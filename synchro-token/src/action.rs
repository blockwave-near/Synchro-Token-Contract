use crate::*;
use near_sdk::{assert_one_yocto, env, log, Promise};
use crate::utils::{GAS_FOR_FT_TRANSFER, GAS_FOR_RESOLVE_TRANSFER, NO_DEPOSIT};

#[near_bindgen]
impl Contract {
    /// unstake token and send assets back to the predecessor account.
    /// Requirements:
    /// * The predecessor account should be registered.
    /// * `amount` must be a positive integer.
    /// * The predecessor account should have at least the `amount` of tokens.
    /// * Requires attached deposit of exactly 1 yoctoNEAR.
    #[payable]
    pub fn unstake(&mut self, amount: U128) -> Promise {
        // Checkpoint
        self.distribute_reward();

        assert_one_yocto();
        let account_id = env::predecessor_account_id();
        let amount: Balance = amount.into();

        assert!(self.ft.total_supply > 0, "ERR_EMPTY_TOTAL_SUPPLY");
        let unlocked = (U256::from(amount) * U256::from(self.locked_token_amount) / U256::from(self.ft.total_supply)).as_u128();

        self.ft.internal_withdraw(&account_id, amount);
        self.locked_token_amount -= unlocked;
        assert!(self.ft.total_supply >= 10u128.pow(18), "ERR_KEEP_AT_LEAST_ONE_SYNCHRO");

        log!("Withdraw {} Sync from {}", amount, account_id);

        ext_fungible_token::ft_transfer(
            account_id.clone(),
            U128(unlocked),
            None,
            &self.locked_token,
            1,
            GAS_FOR_FT_TRANSFER,
        )
            .then(ext_self::callback_post_unstake(
                account_id.clone(),
                U128(unlocked),
                U128(amount),
                &env::current_account_id(),
                NO_DEPOSIT,
                GAS_FOR_RESOLVE_TRANSFER,
            ))
    }

    pub fn spend(&mut self, receiver_id: AccountId, amount: Balance) {
        self.assert_whitelist();
        assert!(amount > 0, "Requires positive attached deposit");
        if !self.ft.accounts.contains_key(&receiver_id) {
            self.ft.internal_register_account(&receiver_id);
        }

        self.ft.internal_transfer(self.owner.into(), receiver_id.into(), amount, None);
    }

    pub fn mint(&mut self, account_id: AccountId, amount: Balance) {
        self.assert_whitelist();

        assert!(amount > 0, "Requires positive attached deposit");
        if !self.ft.accounts.contains_key(&account_id) {
            self.ft.internal_register_account(&account_id);
        }

        self.ft.internal_deposit(&account_id, amount);
        log!("Mint {} Synchro Token to {}", amount, account_id);
    }

    pub fn burn(&mut self, amount: U128) -> Promise {
        assert_one_yocto();
        self.assert_whitelist();
        let account_id = env::predecessor_account_id();
        let amount = amount.into();
        self.ft.internal_withdraw(&account_id, amount);
        log!("Withdraw {} Synchro from {}", amount, account_id);
        // Transferring NEAR and refunding 1 yoctoNEAR.
        Promise::new(account_id).transfer(amount + 1)
    }
}