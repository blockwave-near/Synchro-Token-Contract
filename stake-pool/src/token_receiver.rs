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
    DepositAndStake,
}

#[near_bindgen]
impl FungibleTokenReceiver for StakingContract {
    /// Callback on receiving tokens by this contract.
    /// `msg` format is either "" for deposit or `TokenReceiverMessage`.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        if msg.is_empty() {
            // Simple deposit.
            self.internal_deposit(sender_id.as_ref(), amount.into());
            PromiseOrValue::Value(U128(0))
        } else {
            let message =
                serde_json::from_str::<UserAction>(&msg).expect("ERR_JSON_IS_EMPTY");
            match message {
                UserAction::DepositAndStake => {
                    self.internal_deposit_and_stake(sender_id.as_ref(), amount.into());
                    PromiseOrValue::Value(U128(0))
                },
                _ => {}
            }
        }
    }
}