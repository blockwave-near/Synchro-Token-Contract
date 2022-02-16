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

impl Contract {
    pub(crate) fn assert_whitelist(&self) {
        if !self.whitelist.contains(&env::predecessor_account_id()) {
            env::panic(format!("Not valid account id").as_ref());
        }
    }
}