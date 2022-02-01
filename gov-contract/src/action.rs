use crate::*;

const MIN_TITLE_LENGTH: usize = 4;
const MAX_TITLE_LENGTH: usize = 64;
const MIN_DESC_LENGTH: usize = 4;
const MAX_DESC_LENGTH: usize = 1024;

const ONE_NEAR_YOCTO: Balance =         100_000_000_000_000_000_000_000;
const MIN_VALUE_CREATE_POLL: Balance =  ONE_NEAR_YOCTO * 100; // 100 NEAR

#[near_bindgen]
impl VotingContract {
    /// Contract of VotingContract

    pub fn createPoll(&mut self, title: String, description: String, deposit_amount: Balance) {
        assert!(!self.pause, "This Contract Is Paused");
        assert!(deposit_amount > MIN_VALUE_CREATE_POLL, "Not enough deposit amount");

        let mut new_poll: Poll = Poll {
            creator_id: env::predecessor_account_id(),
            yes_amount: 0,
            no_amount: 0,
            stake_amount: 0,
            status: PollStatus::InProgress,
            title,
            description,
            votes: HashMap::new(),
            last_epoch_height: 0,
            deposit_amount,
            total_balance_at_end_poll: 0
        };
        self.polls.insert(self.poll_count.into(), new_poll);
        self.poll_count += 1;
    }

    /// Contract of PollContract

    /// Ping to update the votes according to current stake of validators.
    pub fn ping(&mut self, amount: Balance ,index: u32) {
        self.check_status(index);
        assert!(index >= 0 && index < self.poll_count, "Not Valid Index");

        let mut cur_poll: Poll = self.polls[index];
        let cur_epoch_height = env::epoch_height();

        if cur_epoch_height != cur_poll.last_epoch_height {
            let votes = std::mem::take(&mut cur_poll.votes);
            cur_poll.stake_amount = 0;
            for (account_id, account_info) in votes {
                match account_info.vote {
                    true => cur_poll.yes_amount += amount,
                    false => cur_poll.no_amount += amount,
                    _ => {}
                }
                cur_poll.stake_amount += amount;
                if amount > 0 {
                    cur_poll.votes
                        .insert(account_id, AccountInfo(account_info.vote, amount));
                }
            }
            self.check_finish(index);
            cur_poll.last_epoch_height = cur_epoch_height;
        }
    }

    /// Check whether the voting has ended.
    fn check_finish(&mut self, index: u32) {
        self.check_status(index);

        let mut cur_poll: Poll = self.polls[index];

        if cur_poll.stake_amount > 2 * cur_poll.stake_amount / 3 {
            if cur_poll.yes_amount > cur_poll.no_amount {
                cur_poll.status = PollStatus::Passed;
            } else {
                cur_poll.status = PollStatus::Rejected;
            }
            self.distribute(index);
        }
    }

    /// Distribute when yes voted
    fn distribute(&mut self, index: u32) {
        self.check_status(index);

        let mut cur_poll: Poll = self.polls[index];

        /// TODO: Create Distribite
        match cur_poll.status {
            PollStatus::Passed => {

            },
            PollStatus::Rejected => {

            },
            _ => env::panic(b"This poll status is InProgress")
        }
    }

    /// Method for validators to vote or withdraw the vote.
    /// Votes for if `is_vote` is true, or withdraws the vote if `is_vote` is false.
    pub fn vote(&mut self, index: u32, account_info: AccountInfo) {
        assert!(vote_amount > MIN_VALUE_VOTE, "{} is not enough amount", vote_amount);
        let account_id = env::predecessor_account_id();

        self.ping(account_info.amount, index);
        self.check_status(index);

        let mut cur_poll: Poll = self.polls[index];

        if vote_amount > 0 {
            cur_poll.votes.insert(account_id, account_info);
            self.check_finish(index);
        }
    }

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
        cur_poll.votes.iter().map(|(account_id, stake)| (account_id.clone(), (*stake).into())).collect()
    }

    // TODO: 트랜잭션이 안끝나는 오류가 생길 시 이 코드 수정
    pub fn check_status(&self, index: u32) {
        let mut cur_poll: Poll = self.polls[index];
        match cur_poll.status {
            PollStatus::Passed => env::panic(b"Voting has already passed"),
            PollStatus::Rejected => env::panic(b"Voting has already rejected"),
            PollStatus::Expired => env::panic(b"Voting has already expired"),
            _ => {}
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum UserAction {
    Vote(AccountInfo),
    CreatePoll(AccountInfo),
}