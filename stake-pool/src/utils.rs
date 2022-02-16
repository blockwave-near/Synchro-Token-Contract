use crate::*;

/// Interface for a voting contract.
#[ext_contract(ext_voting)]
pub trait VoteContract {
    /// Method for validators to vote or withdraw the vote.
    /// Votes for if `is_vote` is true, or withdraws the vote if `is_vote` is false.
    fn vote(&mut self, is_vote: bool);
}

/// Interface for bNear contract.
#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    /// Method for staking pool to mint bNEAR to the delegators when they stakes.
    fn mint(&mut self, account_id: AccountId, amount: Balance);
    /// Method for staking pool to burn bNEAR from the delegators when they unstakes.
    fn burn(&mut self, account_id: AccountId, amount: Balance);
}

/// Interface for the contract itself.
#[ext_contract(ext_self)]
pub trait SelfContract {
    /// A callback to check the result of the staking action.
    /// In case the stake amount is less than the minimum staking threshold, the staking action
    /// fails, and the stake amount is not changed. This might lead to inconsistent state and the
    /// follow withdraw calls might fail. To mitigate this, the contract will issue a new unstaking
    /// action in case of the failure of the first staking action.
    fn on_stake_action(&mut self);

    /// A callback to check the result of the minting action.
    /// In case the account register action couldn't completed due to such as storage staking, the staking
    /// action changes nothing.
    fn on_mint_action(
        &mut self,
        account_id: AccountId,
        charge_amount: Balance,
        num_shares: u128,
    );

    /// A callback to check the result of the burning action.
    /// In case the balance of bNEAR is less than unstaking amount, the unstaking
    /// action changes nothing.
    fn on_burn_action(
        &mut self,
        account_id: AccountId,
        num_shares: u128,
        receive_amount: Balance,
        principal_reduced: Balance,
    );
}

#[near_bindgen]
impl StakingContract {
    /*************/
    /* Callbacks */
    /*************/
    #[private]
    pub fn on_stake_action(&mut self) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );
        let stake_action_succeeded = match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        };

        // If the stake action failed and the current locked amount is positive, then the contract
        // has to unstake.
        if !stake_action_succeeded && env::account_locked_balance() > 0 {
            Promise::new(env::current_account_id()).stake(0, self.stake_public_key.clone());
        }
    }

    #[private]
    pub fn on_mint_action(
        &mut self,
        account_id: AccountId,
        charge_amount: Balance,
        num_shares: u128,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );

        if let PromiseResult::Successful(_) = env::promise_result(0) {
            let mut account = self.internal_get_account(&account_id);

            account.unstaked -= charge_amount;
            account.stake_shares += num_shares;
            account.stake_principal += charge_amount;
            self.internal_save_account(&account_id, &account);
    
            // The staked amount that will be added to the total to guarantee the "stake" share price
            // never decreases. The difference between `stake_amount` and `charge_amount` is paid
            // from the allocated STAKE_SHARE_PRICE_GUARANTEE_FUND.
            let stake_amount = self.staked_amount_from_num_shares_rounded_up(num_shares);
    
            self.total_staked_balance += stake_amount;
            self.total_stake_shares += num_shares;

            env::log(
                format!(
                    "@{} staking {}. Received {} new staking shares. Total {} unstaked balance, {} staking shares, and {} staking principal.",
                    account_id, charge_amount, num_shares, account.unstaked, account.stake_shares, account.stake_principal,
                )
                .as_bytes(),
            );
            env::log(
                format!(
                    "Contract total staked balance is {}. Total number of shares {}",
                    self.total_staked_balance, self.total_stake_shares
                )
                .as_bytes(),
            );
        }
    }

    #[private]
    pub fn on_burn_action(
        &mut self,
        account_id: AccountId,
        num_shares: u128,
        receive_amount: Balance,
        principal_reduced: Balance,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );

        if let PromiseResult::Successful(_) = env::promise_result(0) {
            let mut account = self.internal_get_account(&account_id);

            account.stake_principal -= principal_reduced;

            account.stake_shares -= num_shares;
            account.unstaked += receive_amount;
            account.unstaked_available_epoch_height = env::epoch_height() + NUM_EPOCHS_TO_UNLOCK;

            self.internal_save_account(&account_id, &account);

            // The amount tokens that will be unstaked from the total to guarantee the "stake" share
            // price never decreases. The difference between `receive_amount` and `unstake_amount` is
            // paid from the allocated STAKE_SHARE_PRICE_GUARANTEE_FUND.
            let unstake_amount = self.staked_amount_from_num_shares_rounded_down(num_shares);

            self.total_staked_balance -= unstake_amount;
            self.total_stake_shares -= num_shares;

            env::log(
                format!(
                    "@{} unstaking {}. Spent {} staking shares. Total {} unstaked balance, {} staking shares, and {} staking principal.",
                    account_id, receive_amount, num_shares, account.unstaked, account.stake_shares, account.stake_principal,
                )
                .as_bytes(),
            );
            env::log(
                format!(
                    "Contract total staked balance is {}. Total number of shares {}",
                    self.total_staked_balance, self.total_stake_shares
                )
                .as_bytes(),
            );
        }
    }
}