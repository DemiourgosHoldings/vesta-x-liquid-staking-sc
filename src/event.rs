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
    #[event("UserStake")]
    fn user_stake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] valar_amount: &BigUint,
    );

    #[event("UserUnstake")]
    fn user_unstake_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] valar_amount: &BigUint,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("UserWithdraw")]
    fn user_withdraw_event(
        &self,
        #[indexed] user: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("Donate")]
    fn donate_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    // Admin
    #[event("AdminDelegateSuccess")]
    fn admin_delegate_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
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
        #[indexed] egld_amount: &BigUint,
    );

    #[event("AdminUndelegateFail")]
    fn admin_undelegate_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        err_msg: &ManagedBuffer,
    );

    #[event("AdminWithdrawSuccess")]
    fn admin_withdraw_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    #[event("AdminWithdrawFail")]
    fn admin_withdraw_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    // Rewards
    #[event("AdminRedelegateRewardsSuccess")]
    fn admin_redelegate_rewards_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] rewards_amount: &BigUint,
        #[indexed] fee_stegld_amount: &BigUint,
    );

    #[event("AdminRedelegateRewardsFail")]
    fn admin_redelegate_rewards_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] rewards_amount: &BigUint,
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

    #[event("AdminMoveTreasury")]
    fn admin_move_treasury_event(
        &self,
        #[indexed] to: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
    );

    // Pool
    #[event("UpdateMainPool")]
    fn update_main_pool_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] pool_valar_amount: &BigUint,
        #[indexed] pool_egld_amount: &BigUint,
    );

    // Admin Settings
    #[event("ChangeDelegateAddress")]
    fn change_delegate_address_event(
        &self,
        #[indexed] to: &ManagedAddress,
    );

    #[event("ChangeTreasuryWallet")]
    fn change_treasury_wallet_event(
        &self,
        #[indexed] to: &ManagedAddress,
    );

    #[event("ChangeFee")]
    fn change_fee_event(
        &self,
        #[indexed] fee: u64,
    );
}