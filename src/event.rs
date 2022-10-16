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
    #[event("StakeSuccess")]
    fn stake_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] amount: &BigUint,
    );

    #[event("StakeFail")]
    fn stake_fail_event(
        &self,
    );
}