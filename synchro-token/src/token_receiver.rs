use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    /// Callback on receiving tokens by this contract.
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        // Checkpoint
        self.distribute_reward();
        let token_in = env::predecessor_account_id();
        let amount: Balance = amount.into();
        assert_eq!(token_in, self.locked_token, "ERR_ILLEGAL_TOKEN");
        if msg.is_empty() {
            // user stake.
            self.internal_stake(sender_id.as_ref(), amount);
            PromiseOrValue::Value(U128(0))
        } else {
            // deposit reward
            log!("Add reward {} token with msg {}", amount, msg);
            self.internal_add_reward(sender_id.as_ref(), amount);
            PromiseOrValue::Value(U128(0))
        }
    }
}