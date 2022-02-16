use std::collections::HashMap;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, log, near_bindgen, AccountId, Balance, PanicOnDefault, PromiseOrValue};
use crate::utils::{DURATION_30DAYS_IN_SEC, nano_to_sec};

mod action;
mod owner;
mod utils;

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner: AccountId,
    pub ft: FungibleToken,
    pub whitelist: Vec<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner: AccountId, whitelist: Vec<AccountId>) -> Self {
        Self {
            owner,
            ft: FungibleToken::new(b"a".to_vec()),
            whitelist,
        };
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        let icon_data = "";

        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: "Synchro fungible token".to_string(),
            symbol: "Synchro".to_string(),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}