use near_sdk::{Gas, Promise};
use near_sdk::env::log;
use near_sdk::test_utils::test_env::setup_free;
use crate::*;
use crate::utils::GAS_FOR_FT_TRANSFER;

/// Internal Actions
impl VotingContract {
    /// Main Method

    // Create poll
    pub fn internal_create_poll(&mut self, title: String, description: String, deposit_amount: Balance, sender_id: AccountId) {
        let mut new_poll: Poll = Poll {
            creator_id: sender_id,
            create_date: Some(U64::from(env::block_timestamp())),
            yes_amount: 0,
            no_amount: 0,
            stake_amount: 0,
            status: PollStatus::InProgress,
            title,
            description,
            votes: HashMap::new(),
            last_epoch_height: 0,
            deposit_amount,
            total_balance_at_end_poll: 0,
        };
        self.polls.insert(self.poll_count.into(), new_poll);
        self.poll_count += 1;
    }

    // Method for validators to vote.
    pub fn internal_vote(&mut self, index: u32, account_info: AccountInfo, account_id: AccountId) {
        let mut cur_poll: Poll = self.polls[index];
        let vote = account_info.vote;
        let amount = account_info.amount;
        let mut user_amount = amount.clone();

        if account_info.vote {
            cur_poll.yes_amount += amount;
        } else {
            cur_poll.no_amount += amount;
        }

        user_amount += amount;
        cur_poll.stake_amount = account_info.amount;

        cur_poll.votes.insert(account_id, AccountInfo(vote, user_amount));
        self.check_finish(index, &mut cur_poll);
    }

    /// Sub Method
    // Ping to update the votes according to current stake of validators.
    fn ping(&mut self, amount: Balance, index: u32) {
        let mut cur_poll: Poll = self.polls[index];
        let cur_epoch_height = env::epoch_height();

        if cur_epoch_height != cur_poll.last_epoch_height {
            let votes = std::mem::take(&mut cur_poll.votes);
            cur_poll.stake_amount = 0;
            for (account_id, account_info) in votes {
                match account_info.vote {
                    true => cur_poll.yes_amount += amount,
                    false => cur_poll.no_amount += amount,
                    _ => {
                        env::log(
                            format!("{} is not valid", account_id).as_bytes()
                        );
                    }
                }
                cur_poll.stake_amount += amount;
                cur_poll.votes
                    .insert(account_id, AccountInfo(account_info.vote, amount));
            }
            cur_poll.last_epoch_height = cur_epoch_height;
            self.check_finish(index, &mut cur_poll);
        }
    }

    // Check whether the voting has ended.
    fn check_finish(&mut self, index: u32, cur_poll: &mut Poll) {
        if cur_poll.stake_amount > 2 * cur_poll.stake_amount / 3 {
            /// TODO: 끝내는 조건 필요
            if cur_poll.yes_amount > cur_poll.no_amount {
                cur_poll.status = PollStatus::Passed;
            } else {
                cur_poll.status = PollStatus::Rejected;
            }
        }

        self.polls.insert(index.into(), cur_poll.into());
    }

    /// Called
    pub(crate) fn internal_send_tokens(
        &self,
        sender_id: &AccountId,
        token_id: &AccountId,
        amount: Balance,
    ) -> Promise {
        ext_fungible_token::ft_transfer(
            sender_id.clone(),
            U128(amount),
            None,
            token_id,
            1,
            GAS_FOR_FT_TRANSFER,
        )
    }
}