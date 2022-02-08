mod view;
mod token_receiver;
mod internal;
mod utils;
mod owner;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64, WrappedTimestamp};
use near_sdk::{env, near_bindgen, AccountId, Balance, EpochHeight};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct AccountInfo {
    id: AccountId,
    vote: bool,
    amount: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct VotingContract {
    owner: AccountId,
    polls: Vec<Poll>,
    poll_count: u32,
    pause: bool,
    market_id: AccountId,
    token_id: AccountId,
    min_create_poll_amount: Balance,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Poll {
    creator_id: AccountId,
    status: PollStatus,
    create_date: Option<WrappedTimestamp>,
    title: String,
    description: String,
    deposit_amount: Balance,
    votes: HashMap<AccountId, AccountInfo>,
    yes_amount: Balance,
    no_amount: Balance,
    stake_amount: Balance,
    total_balance_at_end_poll: Balance,
    last_epoch_height: EpochHeight,
}

impl Default for VotingContract {
    fn default() -> Self {
        env::panic(b"Voting contract should be initialized before usage")
    }
}

#[near_bindgen]
impl VotingContract {
    #[init]
    pub fn new(token_id:AccountId, market_id: AccountId, min_create_poll_amount: Balance) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        VotingContract {
            owner: env::predecessor_account_id(),
            polls: Vec::new(),
            poll_count: 0,
            pause: false,
            market_id,
            token_id,
            min_create_poll_amount,
        }
    }

    // claim user
    pub fn claim(&self, index: u32) {
        self.assert_market();
        self.assert_contract_running();
        self.assert_index(index);
        self.check_status(index);

        let mut cur_poll: Poll = self.polls[index];
        let votes = std::mem::take(&mut cur_poll.votes);
        if votes.contains_key(&env::predecessor_account_id()) {
            let amount =
        } else {
            env::panic(b"ERR: non-whitelisted token can NOT deposit into lost-found.");
        };
    }

    pub fn stop_vote(&mut self, index: u32) {
        self.assert_owner();

        let mut cur_poll: Poll = self.polls[index];
        cur_poll.status = PollStatus::Expired;
        self.polls.insert(index.into(), cur_poll);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum PollStatus {
    InProgress,
    Passed,
    Rejected,
    Expired, // Depricated
}
