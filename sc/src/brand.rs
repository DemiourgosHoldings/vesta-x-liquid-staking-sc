elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::constant::{
    TOKEN_ISSUE_COST,
    VEGLD_DISPLAY_NAME,
    VEGLD_TICKER,
    VEGLD_DECIMALS,
};

#[elrond_wasm::module]
pub trait BrandModule:
    crate::common_storage::CommonStorageModule
    + crate::event::EventModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueVegldAndSetAllRoles)]
    fn issue_vegld_and_set_all_roles(
        &self,
    ) {
        require!(
            self.vegld_identifier().is_empty(),
            "vEGLD token is already issued."
        );

        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == TOKEN_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );

        self.vegld_identifier().issue_and_set_all_roles(
            payment_amount,
            ManagedBuffer::new_from_bytes(VEGLD_DISPLAY_NAME),
            ManagedBuffer::new_from_bytes(VEGLD_TICKER),
            VEGLD_DECIMALS,
            Some(self.callbacks().issue_callback()),
        );
    }

    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.vegld_issue_success_event(&token_id, self.blockchain().get_block_timestamp());
                self.vegld_identifier().set_token_id(token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.vegld_issue_fail_event(self.blockchain().get_block_timestamp());
            }
        }
    }
}