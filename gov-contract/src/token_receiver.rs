use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{serde_json, PromiseOrValue};
use near_sdk::json_types::ValidAccountId;

use crate::*;
use crate::action::UserAction;

pub const VIRTUAL_ACC: &str = "@";

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum UserAction {
    CreatePoll{title: String, description: String},
    Vote{index: u32, vote: bool},
}

#[near_bindgen]
impl FungibleTokenReceiver for VotingContract {
    /// Callback on receiving tokens by this contract.
    /// `msg` format is either "" for deposit or `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        self.assert_contract_running();
        self.assert_market(sender_id.to_string());
        let token_in = env::predecessor_account_id();
        if msg.is_empty() {
            env::panic(b"ERR_JSON_IS_EMPTY");
        } else {
            let message =
                serde_json::from_str::<UserAction>(&msg).expect("ERR_JSON_IS_EMPTY");
            match message {
                UserAction::CreatePoll {
                    title,
                    description,
                } => {
                    self.internal_create_poll(title, description, amount.into(), sender_id.to_string());
                    PromiseOrValue::Value(U128(0))
                },
                UserAction::Vote {
                    index,
                    vote
                } => {
                    self.assert_index(index);
                    self.assert_status(index);

                    self.internal_vote(index, vote, amount.into(), sender_id.to_string());
                    PromiseOrValue::Value(U128(0))
                }
            }
        }
    }
}