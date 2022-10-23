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

    // User Activities
    #[event("Prestake")]
    fn prestake_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] auto_delegate_enabled: bool,
    );

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
        #[indexed] from: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        err_msg: &ManagedBuffer,
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

    #[event("Claim")]
    fn claim_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    // Admin
    #[event("RedelegateRewardsSuccess")]
    fn redelegate_rewards_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
    );
    
    #[event("RedelegateRewardsFail")]
    fn redelegate_rewards_fail_event(
        &self,
    );

    #[event("WithdrawSuccess")]
    fn withdraw_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );
    
    #[event("WithdrawFail")]
    fn withdraw_fail_event(
        &self,
    );

    #[event("ClaimRewardsSuccess")]
    fn claim_rewards_success_event(
        &self,
        #[indexed] from: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );
    
    #[event("ClaimRewardsFail")]
    fn claim_rewards_fail_event(
        &self,
    );

    // Admin Settings
    #[event("ChangeDelegateAddress")]
    fn change_delegate_address(
        &self,
        #[indexed] to: &ManagedAddress,
    );

    #[event("ChangeTreasuryWallet")]
    fn change_treasury_wallet(
        &self,
        #[indexed] to: &ManagedAddress,
    );

    #[event("ChangeAutoDelegateEnabled")]
    fn change_auto_delegate_enabled(
        &self,
        #[indexed] value: bool,
    );
}