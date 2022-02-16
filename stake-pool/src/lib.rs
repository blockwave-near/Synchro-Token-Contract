use std::convert::TryInto;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::json_types::{Base58PublicKey, U128, ValidAccountId};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, EpochHeight, Promise, PublicKey,
    ext_contract, PromiseResult, Gas, assert_one_yocto,
};
use uint::construct_uint;

mod internal;
mod owner;
mod utils;
mod views;
mod token_receiver;

use crate::utils::{ext_voting, ext_fungible_token, ext_self};

/// The amount of gas given to complete `vote` call.
const VOTE_GAS: Gas = 100_000_000_000_000;

/// The amount of gas given to complete internal `on_stake_action` call.
const ON_STAKE_ACTION_GAS: Gas = 20_000_000_000_000;

/// The amount of gas given to complete internal `on_burn_action` call.
const ON_MINT_AND_BURN_ACTION_GAS: Gas = 100_000_000_000_000;

/// The amount of gas given to complete 'mint' and 'burn' call.
const MINT_AND_BURN_GAS: Gas = 20_000_000_000_000;

/// The amount of yocto NEAR the contract dedicates to guarantee that the "share" price never
/// decreases. It's used during rounding errors for share -> amount conversions.
const STAKE_SHARE_PRICE_GUARANTEE_FUND: Balance = 1_000_000_000_000;

/// There is no deposit balance attached.
const NO_DEPOSIT: Balance = 0;

/// A type to distinguish between a balance and "stake" shares for better readability.
pub type NumStakeShares = Balance;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

/// Inner account data of a delegate.
#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct Account {
    pub unstaked: Balance,
    pub unstaked_available_epoch_height: EpochHeight,
    pub stake_principal: Balance,
}

/// Represents an account structure readable by humans.
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HumanReadableAccount {
    pub account_id: AccountId,
    pub unstaked_balance: U128,
    pub staked_balance: U128,
    pub can_withdraw: bool,
    pub stake_reward: U128,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            unstaked: 0,
            unstaked_available_epoch_height: 0,
            stake_principal: 0,
        }
    }
}

/// The number of epochs required for the locked balance to become unlocked.
/// NOTE: The actual number of epochs when the funds are unlocked is 3. But there is a corner case
/// when the unstaking promise can arrive at the next epoch, while the inner state is already
/// updated in the previous epoch. It will not unlock the funds for 4 epochs.
const NUM_EPOCHS_TO_UNLOCK: EpochHeight = 4;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct StakingContract {
    pub owner_id: AccountId,
    pub stake_public_key: PublicKey,
    pub last_epoch_height: EpochHeight,
    pub last_total_balance: Balance,
    pub total_staked_balance: Balance,
    pub reward_fee_fraction: RewardFeeFraction,
    pub accounts: UnorderedMap<AccountId, Account>,
    pub paused: bool,
    pub token_contract: AccountId,
}

impl Default for StakingContract {
    fn default() -> Self {
        env::panic(b"Staking contract should be initialized before usage")
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct RewardFeeFraction {
    pub numerator: u32,
    pub denominator: u32,
}

impl RewardFeeFraction {
    pub fn assert_valid(&self) {
        assert_ne!(self.denominator, 0, "Denominator must be a positive number");
        assert!(
            self.numerator <= self.denominator,
            "The reward fee must be less or equal to 1"
        );
    }

    pub fn multiply(&self, value: Balance) -> Balance {
        (U256::from(self.numerator) * U256::from(value) / U256::from(self.denominator)).as_u128()
    }
}

#[near_bindgen]
impl StakingContract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        stake_public_key: Base58PublicKey,
        reward_fee_fraction: RewardFeeFraction,
        token_contract: ValidAccountId,
    ) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        reward_fee_fraction.assert_valid();
        assert!(
            env::is_valid_account_id(owner_id.as_bytes()),
            "The owner account ID is invalid"
        );

        Self {
            owner_id,
            stake_public_key: stake_public_key.into(),
            last_epoch_height: env::epoch_height(),
            last_total_balance: 0,
            total_staked_balance: 0,
            reward_fee_fraction,
            accounts: UnorderedMap::new(b"u".to_vec()),
            paused: false,
            token_contract: token_contract.into(),
        };
    }

    /// Withdraws the entire unstaked balance from the predecessor account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    #[payable]
    pub fn withdraw_all(&mut self) {
        assert_one_yocto();
        self.internal_ping();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        self.internal_withdraw(account.unstaked);
    }

    /// Withdraws the non staked balance for given account.
    /// It's only allowed if the `unstake` action was not performed in the four most recent epochs.
    #[payable]
    pub fn withdraw(&mut self, amount: U128) {
        assert_one_yocto();
        self.internal_ping();

        let amount: Balance = amount.into();
        self.internal_withdraw(amount);
    }

    /// Stakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough unstaked balance.
    #[payable]
    pub fn stake(&mut self, amount: U128) {
        assert_one_yocto();
        self.internal_ping();

        let amount: Balance = amount.into();
        self.internal_stake(amount);
    }

    /// Stakes all available unstaked balance from the inner account of the predecessor.
    #[payable]
    pub fn stake_all(&mut self) {
        assert_one_yocto();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        self.internal_stake(account.unstaked);
    }

    /// Unstakes all staked balance from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    #[payable]
    pub fn unstake_all(&mut self) {
        assert_one_yocto();
        self.internal_ping();

        let account_id = env::predecessor_account_id();
        let account = self.internal_get_account(&account_id);
        let amount = self.staked_amount_from_num_shares_rounded_down(account.stake_shares);
        self.inner_unstake(amount);
    }

    /// Unstakes the given amount from the inner account of the predecessor.
    /// The inner account should have enough staked balance.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    #[payable]
    pub fn unstake(&mut self, amount: U128) {
        assert_one_yocto();
        self.internal_ping();

        let amount: Balance = amount.into();
        self.inner_unstake(amount);
    }

    /// Unstakes all stake reward from the inner account of the predecessor.
    /// The new total unstaked balance will be available for withdrawal in four epochs.
    #[payable]
    pub fn unstake_reward(&mut self) {
        assert_one_yocto();
        self.internal_ping();

        let account_id = env::predecessor_account_id();
        let amount: Balance = self.internal_get_stake_reward(&account_id);
        self.inner_unstake(amount);
    }
}