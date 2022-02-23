//! Implement all the relevant logic for owner of this contract.

use crate::*;

#[near_bindgen]
impl Contract {
    /// Get the owner of this account.
    pub fn get_owner(&self) -> AccountId {
        self.owner.clone()
    }

    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "ERR_NOT_ALLOWED"
        );
    }

    /// Change owner. Only can be called by owner.
    pub fn set_owner(&mut self, owner_id: ValidAccountId) {
        self.assert_owner();
        self.owner = owner_id.as_ref().clone();
    }

    pub fn set_whitelist(&mut self, new_whitelist: Vec<AccountId>) {
        self.assert_whitelist();

        self.whitelist = new_whitelist;
    }

    pub fn set_locked_token(&mut self, new_token_id: ValidAccountId) {
        self.assert_whitelist();

        self.locked_token = new_token_id.into();
    }

    pub fn set_reward_per_sec(&mut self, new_reward: Balance) {
        self.assert_whitelist();

        self.reward_per_sec = new_reward;
    }
}