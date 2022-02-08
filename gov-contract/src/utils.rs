use near_sdk::Gas;
use crate::*;

pub const MIN_SEED_DEPOSIT: u128 = 1_000_000_000_000_000_000;
pub const MAX_ACCOUNT_LENGTH: u128 = 64;
/// Amount of gas for fungible token transfers.
pub const GAS_FOR_FT_TRANSFER: Gas = 10_000_000_000_000;
/// Amount of gas for reward token transfers resolve.
pub const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;
/// Amount of gas for seed token transfers resolve.
pub const GAS_FOR_RESOLVE_WITHDRAW_SEED: Gas = 80_000_000_000_000;

pub const INITIAL_MIN_CREATE_POLL: Balance = 100_000_000_000_000_000_000_000_000;

/// TODO: this should be in the near_standard_contracts
#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}

impl VotingContract {
    pub(crate) fn assert_market(&self, account: AccountId) {
        assert_eq!(self.market_id, account, "ERR_NOT_VALID_ACCOUNT");
    }

    pub(crate) fn assert_contract_running(&self) {
        assert!(!self.pause, "ERR_PAUSED_CONTRACT");
    }

    pub(crate) fn assert_index(&self, index: u32) {
        assert!(index >= 0 && index < self.poll_count, "ERR_NOT_VALID_POLL_INDEX");
    }

    pub(crate) fn assert_status(&self, index: u32) {
        let cur_poll: Poll = self.polls[index];
        match cur_poll.status {
            PollStatus::Passed => env::panic(b"Voting has already passed"),
            PollStatus::Rejected => env::panic(b"Voting has already rejected"),
            PollStatus::Expired => env::panic(b"Voting has already expired"),
            _ => {}
        }
    }
}