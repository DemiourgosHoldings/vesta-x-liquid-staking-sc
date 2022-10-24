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
    #[event("UserPrestake")]
    fn user_prestake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] auto_delegate_enabled: bool,
    );

    #[event("UserDelegateSuccess")]
    fn user_delegate_success_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("UserDelegateFail")]
    fn user_delegate_fail_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        err_msg: &ManagedBuffer,
    );

    #[event("UserPreunstake")]
    fn user_preunstake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("Claim")]
    fn claim_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    // Admin
    #[event("AdminDelegateSuccess")]
    fn admin_delegate_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("AdminDelegateFail")]
    fn admin_delegate_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        err_msg: &ManagedBuffer,
    );

    #[event("AdminUndelegateSuccess")]
    fn admin_undelegate_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("AdminUndelegateFail")]
    fn admin_undelegate_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
        err_msg: &ManagedBuffer,
    );

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