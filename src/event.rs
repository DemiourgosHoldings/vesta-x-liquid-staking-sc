elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventModule
{
    #[event("ValarIssueSuccess")]
    fn valar_issue_success_event(
        &self,
        #[indexed] token_id: &TokenIdentifier,
    );

    #[event("ValarIssueFail")]
    fn valar_issue_fail_event(
        &self,
    );
}