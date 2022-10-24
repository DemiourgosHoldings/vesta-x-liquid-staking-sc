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

    #[event("UserWithdraw")]
    fn user_withdraw_event(
        &self,
        #[indexed] user: &ManagedAddress,
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

    #[event("AdminUnbondEvent")]
    fn admin_unbond_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] target_unbond_egld_amount: &BigUint,
        #[indexed] real_unbond_egld_amount: &BigUint,
    );

    // Rewards
    #[event("AdminRedelegateRewardsSuccess")]
    fn admin_redelegate_rewards_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
    );

    #[event("AdminRedelegateRewardsFail")]
    fn admin_redelegate_rewards_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
    );

    #[event("AdminClaimRewardsSuccess")]
    fn admin_claim_rewards_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
    );

    #[event("AdminClaimRewardsFail")]
    fn admin_claim_rewards_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
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