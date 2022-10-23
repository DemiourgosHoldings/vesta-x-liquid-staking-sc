elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{
    TOKEN_ISSUE_COST,
    VALAR_DISPLAY_NAME,
    VALAR_TICKER,
    VALAR_DECIMALS,
};

#[elrond_wasm::module]
pub trait ValarModule:
    crate::common_storage::StorageModule
    + crate::event::EventModule
{
    #[only_owner]
    #[payable("EGLD")]
    #[endpoint(issueValarAndSetAllRoles)]
    fn issue_valar_and_set_all_roles(
        &self,
    ) {
        require!(
            self.valar_identifier().is_empty(),
            "Valar token is already issued."
        );

        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == TOKEN_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );

        self.valar_identifier().issue_and_set_all_roles(
            payment_amount,
            ManagedBuffer::new_from_bytes(VALAR_DISPLAY_NAME),
            ManagedBuffer::new_from_bytes(VALAR_TICKER),
            VALAR_DECIMALS,
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
                self.valar_issue_success_event(&token_id);
                self.valar_identifier().set_token_id(token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.valar_issue_fail_event();
            }
        }
    }
}