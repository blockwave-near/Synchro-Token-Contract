use near_sdk::json_types::U128;
use near_sdk::{AccountId, ext_contract, Gas, PromiseResult, Timestamp};
use uint::construct_uint;
use crate::*;

/// Attach no deposit.
pub const NO_DEPOSIT: u128 = 0;

pub const GAS_FOR_RESOLVE_TRANSFER: Gas = 10_000_000_000_000;

pub const GAS_FOR_FT_TRANSFER: Gas = 20_000_000_000_000;

pub const DURATION_30DAYS_IN_SEC: u32 = 60 * 60 * 24 * 30;


construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}

pub fn nano_to_sec(nano: Timestamp) -> u32 {
    (nano / 1_000_000_000) as u32
}

#[ext_contract(ext_self)]
pub trait Synchro {
    fn callback_post_unstake(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        share: U128,
    );
}

impl Contract {
    pub(crate) fn assert_whitelist(&self) {
        if !self.whitelist.contains(&env::predecessor_account_id()) {
            env::panic(format!("Not valid account id").as_ref());
        }
    }

    #[private]
    pub fn callback_post_unstake(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        share: U128,
    ) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Err: expected 1 promise result from unstake"
        );
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Successful(_) => {}
            PromiseResult::Failed => {
                // This reverts the changes from unstake function.
                // If account doesn't exit, the unlock token stay in contract.
                if self.ft.accounts.contains_key(&sender_id) {
                    self.locked_token_amount += amount.0;
                    self.ft.internal_deposit(&sender_id, share.0);
                    env::log(
                        format!(
                            "Account {} unstake failed and reverted.",
                            sender_id
                        )
                            .as_bytes(),
                    );
                } else {
                    env::log(
                        format!(
                            "Account {} has unregisterd. unlocking token goes to contract.",
                            sender_id
                        )
                            .as_bytes(),
                    );
                }
            }
        };
    }
}