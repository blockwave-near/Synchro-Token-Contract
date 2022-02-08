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

    pub fn change_min_create_poll_amount(&mut self, amount: Balance) {
        self.assert_owner();

        self.min_create_poll_amount = amount;
    }

    pub fn pause_staking(&mut self) {
        self.assert_owner();
        assert!(!self.paused, "The staking is already paused");

        self.pause = true;
    }

    pub fn resume_staking(&mut self) {
        self.assert_owner();
        assert!(self.paused, "The staking is not paused");

        self.pause = false;
    }
}