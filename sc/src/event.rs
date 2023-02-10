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

    #[event("WithdrawFromPrestaked")]
    fn withdraw_from_prestaked_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    // Admin
    #[event("DelegateToStakingProviderSuccess")]
    fn delegate_to_staking_provider_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("DelegateToStakingProviderFail")]
    fn delegate_to_staking_provider_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
        err_msg: &ManagedBuffer,
    );

    #[event("UndelegateFromStakingProviderSuccess")]
    fn undelegate_from_staking_provider_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("UndelegateFromStakingProviderFail")]
    fn undelegate_from_staking_provider_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
        err_msg: &ManagedBuffer,
    );

    #[event("WithdrawFromStakingProviderSuccess")]
    fn withdraw_from_staking_provider_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("WithdrawFromStakingProviderFail")]
    fn withdraw_from_staking_provider_fail_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] timestamp: u64,
    );

    // Rewards
    #[event("ClaimRewardsFromStakingProviderSuccess")]
    fn claim_rewards_from_staking_provider_success_event(
        &self,
        #[indexed] caller: &ManagedAddress,
        #[indexed] delegate_address: &ManagedAddress,
        #[indexed] rewards_egld_amount: &BigUint,
        #[indexed] timestamp: u64,
    );

    #[event("ClaimRewardsFromStakingProviderFail")]
    fn claim_rewards_from_staking_provider_fail_event(
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