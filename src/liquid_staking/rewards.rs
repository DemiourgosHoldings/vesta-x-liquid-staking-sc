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
    fn admin_redelegate_rewards(&self, delegate_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address.clone())
            .reDelegateRewards()
            .async_call()
            .with_callback(self.callbacks().redelegate_rewards_callback(&caller, &delegate_address))
            .call_and_exit();
    }

    #[callback]
    fn redelegate_rewards_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.admin_redelegate_rewards_success_event(caller, delegate_address);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.admin_redelegate_rewards_fail_event(caller, delegate_address);
            },
        }
    }

    //
    #[only_owner]
    #[endpoint(adminClaimRewards)]
    fn admin_claim_rewards(&self, delegate_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        self.delegate_contract(delegate_address.clone())
            .claimRewards()
            .async_call()
            .with_callback(self.callbacks().claim_rewards_callback(&caller, &delegate_address))
            .call_and_exit();
    }

    #[callback]
    fn claim_rewards_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,

    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.admin_claim_rewards_success_event(caller, delegate_address);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.admin_claim_rewards_fail_event(caller, delegate_address);
            },
        }
    }
    
}