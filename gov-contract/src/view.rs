use crate::*;

#[near_bindgen]
impl VotingContract {
    /// Get the timestamp of when the voting finishes. `None` means the voting hasn't ended yet.
    pub fn get_result(&self, index: u32) -> PollStatus {
        let mut cur_poll: Poll = self.polls[index];
        cur_poll.status.clone()
    }

    /// Returns current a pair of `total_voted_stake` and the total stake.
    /// Note: as a view method, it doesn't recompute the active stake. May need to call `ping` to
    /// update the active stake.
    pub fn get_total_voted_stake(&self, index: u32) -> U128 {
        let mut cur_poll: Poll = self.polls[index];
        cur_poll.stake_amount;
    }

    /// Returns all active votes.
    /// Note: as a view method, it doesn't recompute the active stake. May need to call `ping` to
    /// update the active stake.
    pub fn get_votes(&self, index: u32) -> HashMap<AccountId, U128> {
        let mut cur_poll: Poll = self.polls[index];
        cur_poll.votes.iter().map(|(account_id, info)| (account_id.clone(), info.vote, (*info.amount).into())).collect()
    }
}