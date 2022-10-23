elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate_proxy;

#[elrond_wasm::module]
pub trait RewardsModule:
crate::storages::common_storage::CommonStorageModule
+ crate::storages::pool_storage::PoolStorageModule
+ crate::event::EventModule
+ crate::pool::PoolModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate_proxy::Proxy<Self::Api>;
    
    #[only_owner]
    #[endpoint(adminRedelegateRewards)]
    fn admin_redelegate_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();

        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        self.delegate_contract(delegate_address.clone())
            .reDelegateRewards()
            .async_call()
            .with_callback(self.callbacks().redelegate_rewards_callback(&caller))
            .call_and_exit();
    }

    #[callback]
    fn redelegate_rewards_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.redelegate_rewards_success_event(caller);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.redelegate_rewards_fail_event();
            },
        }
    }

    //
    #[only_owner]
    #[endpoint(adminWithdraw)]
    fn admin_withdraw(&self) {
        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();

        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        self.delegate_contract(delegate_address.clone())
            .withdraw()
            .async_call()
            .with_callback(self.callbacks().withdraw_callback(&caller))
            .call_and_exit();
    }

    #[callback]
    fn withdraw_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                //
                // self.unbonded_egld_amount().update(|v| *v += &egld_amount);

                self.withdraw_success_event(caller, &BigUint::zero());
            },
            ManagedAsyncCallResult::Err(_) => {
                self.withdraw_fail_event();
            },
        }
    }

    //
    #[only_owner]
    #[endpoint(adminClaimRewards)]
    fn admin_claim_rewards(&self) {
        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();

        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        self.delegate_contract(delegate_address.clone())
            .claimRewards()
            .async_call()
            .with_callback(self.callbacks().claim_rewards_callback(&caller))
            .call_and_exit();
    }

    #[callback]
    fn claim_rewards_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.claim_rewards_success_event(caller, &BigUint::zero());
            },
            ManagedAsyncCallResult::Err(_) => {
                self.claim_rewards_fail_event();
            },
        }
    }
    
}