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

pub const MIN_CREATE_POLL: Gas = 100_000_000_000_000_000_000_000_000;

/// TODO: this should be in the near_standard_contracts
#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>);
}