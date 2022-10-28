elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::cmp::min;

use crate::delegate_proxy;
use crate::config::{ DELEGATE_MIN_AMOUNT };

#[elrond_wasm::module]
pub trait AdminModule:
crate::storages::common_storage::CommonStorageModule
+ crate::storages::pool_storage::PoolStorageModule
+ crate::event::EventModule
+ crate::amm::AmmModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate_proxy::Proxy<Self::Api>;
    
    #[only_owner]
    #[endpoint(adminDelegate)]
    fn admin_delegate(
        &self,
        delegate_address: ManagedAddress,
        opt_amount: OptionalValue<BigUint>,
    ) {
        // if amount is not given, delegate total prestaked amount
        let delegating_amount = match opt_amount {
            OptionalValue::Some(v) => min(v, self.prestaked_egld_amount().get()),
            OptionalValue::None => self.prestaked_egld_amount().get(),
        };

        require!(
            delegating_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "delegating_amount cannot be less than 1 EGLD."
        );
        require!(
            delegating_amount <= self.blockchain().get_balance(&self.blockchain().get_sc_address()),
            "Not enough EGLD in Smart Contract."
        );

        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address.clone())
            .delegate()
            .with_egld_transfer(delegating_amount.clone())
            .async_call()
            .with_callback(self.callbacks().admin_delegate_callback(&caller, &delegate_address, &delegating_amount))
            .call_and_exit();
    }

    #[callback]
    fn admin_delegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        delegated_amount: &BigUint,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // update Prestake Pool
                self.prestaked_egld_amount().update(|v| *v -= delegated_amount);
                self.admin_delegate_success_event(caller, delegate_address, delegated_amount);
            },
            ManagedAsyncCallResult::Err(err) => {
                self.admin_delegate_fail_event(caller, delegate_address, delegated_amount, &err.err_msg);
            },
        }
    }

    #[only_owner]
    #[endpoint(adminUndelegate)]
    fn admin_undelegate(
        &self,
        delegate_address: ManagedAddress,
        opt_amount: OptionalValue<BigUint>,
    ) {
        // if amount is not given, undelegate total preunstaked amount
        let undelegating_amount = match opt_amount {
            OptionalValue::Some(v) => min(v, self.preunstaked_egld_amount().get()),
            OptionalValue::None => self.preunstaked_egld_amount().get(),
        };

        require!(
            undelegating_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "undelegating_amount cannot be less than 1 EGLD."
        );

        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address.clone())
            .unDelegate(undelegating_amount.clone())
            .async_call()
            .with_callback(self.callbacks().admin_undelegate_callback(
                &caller,
                &delegate_address,
                &undelegating_amount,
            ))
            .call_and_exit();
    }

    #[callback]
    fn admin_undelegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        undelegated_amount: &BigUint,
    ){
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.preunstaked_egld_amount().update(|v| *v -= undelegated_amount);
                self.unstaking_egld_amount().update(|v| *v += undelegated_amount);

                self.admin_undelegate_success_event(caller, delegate_address, undelegated_amount);
            },
            ManagedAsyncCallResult::Err(err) => {
                self.admin_undelegate_fail_event(caller, delegate_address, undelegated_amount, &err.err_msg);
            },
        } 
    }

    ///
    #[only_owner]
    #[endpoint(adminWithdraw)]
    fn admin_withdraw(&self, delegate_address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        self.delegate_contract(delegate_address.clone())
            .withdraw()
            .async_call()
            .with_callback(self.callbacks().withdraw_callback(&caller, &delegate_address))
            .call_and_exit();
    }

    #[callback]
    fn withdraw_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.admin_withdraw_success_event(caller, delegate_address);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.admin_withdraw_fail_event(caller, delegate_address);
            },
        }
    }

    ///
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

    ///
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