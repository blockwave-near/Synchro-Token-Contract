mod action;
mod view;
mod token_receiver;
mod internal;
mod utils;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, EpochHeight};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub struct AccountInfo {
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
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Poll {
    creator_id: AccountId,
    status: PollStatus,
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
    pub fn new() -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");
        VotingContract {
            owner: env::predecessor_account_id(),
            polls: Vec::new(),
            poll_count: 0,
            pause: false,
        }
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
