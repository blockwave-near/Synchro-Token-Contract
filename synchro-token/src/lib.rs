use std::collections::HashMap;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use crate::utils::{DURATION_30DAYS_IN_SEC, nano_to_sec};

mod action;
mod owner;
mod utils;
mod internal;
mod token_receiver;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner: AccountId,
    pub ft: FungibleToken,
    pub whitelist: Vec<AccountId>,
    /// Synchro token
    pub locked_token: AccountId,
    /// deposit reward that does not distribute to locked REF yet
    pub undistributed_reward: Balance,
    /// locked amount
    pub locked_token_amount: Balance,
    /// the previous distribution time in seconds
    pub prev_distribution_time_in_sec: u32,
    /// when would the reward starts to distribute
    pub reward_genesis_time_in_sec: u32,
    pub reward_per_sec: Balance,
    /// current account number in contract
    pub account_number: u64,
}

#[near_bindgen]
impl Contract {

    #[init]
    pub fn new(whitelist: Vec<AccountId>, locked_token: ValidAccountId) -> Self {
        let initial_reward_genisis_time = DURATION_30DAYS_IN_SEC + nano_to_sec(env::block_timestamp());
        let mut contract = Self {
            owner: env::predecessor_account_id(),
            ft: FungibleToken::new(b"a".to_vec()),
            whitelist,
            locked_token: locked_token.into(),
            undistributed_reward: 0,
            locked_token_amount: 0,
            prev_distribution_time_in_sec: initial_reward_genisis_time,
            reward_genesis_time_in_sec: initial_reward_genisis_time,
            reward_per_sec: 0,
            account_number: 0,
        };

        contract.ft.internal_deposit(&env::current_account_id(), 100_000_000_000_000);

        contract
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let icon_data = "";

        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Synchro fungible token".to_string(),
            symbol: "Synchro".to_string(),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}