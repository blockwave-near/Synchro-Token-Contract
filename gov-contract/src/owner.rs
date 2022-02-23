use near_sdk::json_types::ValidAccountId;
use crate::*;

#[near_bindgen]
impl VotingContract {
    /// Owner
    pub(crate) fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner,
            "ERR_NOT_ALLOWED_OWNER"
        );
    }

    pub fn pause_contract(&mut self, pause: bool) {
        self.assert_owner();
        assert_ne!(self.paused, pause, "The contract is already that state");

        self.paused = pause;
    }

    pub fn set_owner(&mut self, new_owner: ValidAccountId) {
        self.assert_owner();

        self.owner = new_owner.into();
    }

    pub fn set_market_id(&mut self, account: ValidAccountId) {
        self.assert_owner();

        self.market_id = account.into();
    }

    pub fn set_token_id(&mut self, account: ValidAccountId) {
        self.assert_owner();

        self.token_id = account.into();
    }

    pub fn set_min_create_poll_amount(&mut self, amount: Balance) {
        self.assert_owner();

        self.min_create_poll_amount = amount;
    }
}