use crate::*;
use near_sdk::{assert_one_yocto, env, log, Promise};
use crate::utils::{GAS_FOR_FT_TRANSFER, GAS_FOR_RESOLVE_TRANSFER, NO_DEPOSIT};

#[near_bindgen]
impl Contract {
    pub fn spend(&mut self, receiver_id: AccountId, amount: Balance) {
        self.assert_whitelist();

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
