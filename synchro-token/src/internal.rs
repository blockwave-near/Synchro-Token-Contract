use std::cmp::{max, min};
use crate::*;

impl Contract {
    pub fn internal_stake(&mut self, account_id: &AccountId, amount: Balance) {
        // check account has registered
        assert!(self.ft.accounts.contains_key(account_id), "Account not registered.");

        if self.ft.total_supply != 0 {
            assert!(self.locked_token_amount > 0, "ERR_INTERNAL");
        }

        self.locked_token_amount += amount;
    }

    pub fn internal_add_reward(&mut self, account_id: &AccountId, amount: Balance) {
        self.undistributed_reward += amount;
        log!("{} add {} assets as reward", account_id, amount);
    }

    /// return the amount of to be distribute reward this time
    pub(crate) fn try_distribute_reward(&self, cur_timestamp_in_sec: u32) -> Balance {
        if cur_timestamp_in_sec > self.reward_genesis_time_in_sec && cur_timestamp_in_sec > self.prev_distribution_time_in_sec {
            let ideal_amount = self.reward_per_sec * (cur_timestamp_in_sec - self.prev_distribution_time_in_sec) as u128;
            min(ideal_amount, self.undistributed_reward)
        } else {
            0
        }
    }

    pub(crate) fn distribute_reward(&mut self) {
        let cur_time = nano_to_sec(env::block_timestamp());
        let new_reward = self.try_distribute_reward(cur_time);
        if new_reward > 0 {
            self.undistributed_reward -= new_reward;
            self.locked_token_amount += new_reward;
        }
        self.prev_distribution_time_in_sec = max(cur_time, self.reward_genesis_time_in_sec);
    }
}