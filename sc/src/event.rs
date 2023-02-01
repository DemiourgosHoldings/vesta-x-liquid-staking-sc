elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait EventModule
{
    // Vegld Token
    #[event("VegldIssueSuccess")]
    fn vegld_issue_success_event(
        &self,
        #[indexed] token_id: &TokenIdentifier,
        #[indexed] timestamp: u64,
    );

    #[event("VegldIssueFail")]
    fn vegld_issue_fail_event(
        &self,
        #[indexed] timestamp: u64,
    );

    // User Activities
    #[event("UserStake")]
    fn user_stake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] vegld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("UserUnstake")]
    fn user_unstake_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] vegld_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("UserWithdraw")]
    fn user_withdraw_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("Donate")]
    fn donate_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("FastWithdraw")]
    fn emit_fast_withdraw_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    // Admin
    #[event("AdminDelegateSuccess")]
    fn admin_delegate_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("AdminDelegateFail")]
    fn admin_delegate_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
        err_msg: &ManagedBuffer,
    );

    #[event("AdminUndelegateSuccess")]
    fn admin_undelegate_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("AdminUndelegateFail")]
    fn admin_undelegate_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
        err_msg: &ManagedBuffer,
    );

    #[event("AdminWithdrawSuccess")]
    fn admin_withdraw_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("AdminWithdrawFail")]
    fn admin_withdraw_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] timestamp: u64,
    );

    // Rewards
    #[event("AdminClaimRewardsSuccess")]
    fn emit_admin_claim_rewards_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] rewards_egld_amount: &BigUint,
        #[indexed] fee_egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("AdminClaimRewardsFail")]
    fn emit_admin_claim_rewards_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] timestamp: u64,
    );

    // Admin Settings
    #[event("ChangeTreasuryWallet")]
    fn change_treasury_wallet_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] to: &ManagedAddress,
        #[indexed] timestamp: u64,
    );

    #[event("ChangeFee")]
    fn change_fee_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] fee: u64,
        #[indexed] timestamp: u64,
    );
}