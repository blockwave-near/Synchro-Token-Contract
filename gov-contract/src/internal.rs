use near_sdk::{Gas, Promise};
use crate::*;
use crate::utils::GAS_FOR_FT_TRANSFER;

/// Internal Actions
impl VotingContract {
    pub(crate) fn internal_distribute(
        &self,
        index: u32,
    ) {
        let mut cur_poll = self.polls[index];

    }

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