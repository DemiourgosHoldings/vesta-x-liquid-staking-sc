elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventModule
{
    // Valar Token
    #[event("ValarIssueSuccess")]
    fn valar_issue_success_event(
        &self,
        #[indexed] token_id: &TokenIdentifier,
    );

    #[event("ValarIssueFail")]
    fn valar_issue_fail_event(
        &self,
    );

    // Stake
    #[event("DelegateSuccess")]
    fn delegate_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("DelegateFail")]
    fn delegate_fail_event(
        &self,
    );

    #[event("UndelegateSuccess")]
    fn undelegate_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("UndelegateFail")]
    fn undelegate_fail_event(
        &self,
    );
}