elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use core::cmp::min;

use crate::delegate_proxy;
use crate::constant::{ DELEGATE_MIN_AMOUNT, MAX_PERCENTAGE, MIN_GAS_FOR_ASYNC_CALL, MIN_GAS_FOR_CALLBACK, ERROR_INSUFFICIENT_GAS, MAX_BLOCKS_FOR_ASYNC_CALLBACK };

#[elrond_wasm::module]
pub trait AdminModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::event::EventModule
    + crate::amm::AmmModule
    + crate::validation::ValidationModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate_proxy::Proxy<Self::Api>;
    
    #[endpoint(adminDelegate)]
    fn admin_delegate(
        &self,
        delegate_address_opt: OptionalValue<ManagedAddress>,
        opt_amount: OptionalValue<BigUint>,
    ) {
        self.require_is_owner_or_admin();
        self.require_admin_action_allowed();
        self.require_initial_configuration_is_done();

        // use auto_delegate_address if delegate_address_opt is None
        let delegate_address = match delegate_address_opt {
            OptionalValue::Some(v) => v,
            OptionalValue::None => {
                require!(
                    !self.auto_delegate_address().is_empty(),
                    "auto_delegate_address is empty"
                );
                self.auto_delegate_address().get()
            }
        };

        // if amount is not given, delegate total prestaked amount
        let delegating_amount = match opt_amount {
            OptionalValue::Some(v) => min(v, self.prestaked_egld_amount().get()),
            OptionalValue::None => self.prestaked_egld_amount().get(),
        };

        self._delegate(delegate_address, delegating_amount);
    }

    #[inline]
    fn _delegate(
        &self,
        delegate_address: ManagedAddress,
        delegating_amount: BigUint,
    ) {
        require!(
            delegating_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "delegating_amount cannot be less than 1 EGLD."
        );
        require!(
            delegating_amount <= self.blockchain().get_balance(&self.blockchain().get_sc_address()),
            "Not enough EGLD in Smart Contract."
        );
        self.require_whitelisted_staking_provider(&delegate_address);

        // update Prestake Pool
        self.prestaked_egld_amount().update(|v| *v -= &delegating_amount);

        let caller = self.blockchain().get_caller();
        let gas_for_async_call = self.get_gas_for_async_call();

        // for async call check
        self.last_async_call_id().update(|v| *v += 1);
        self.async_call_start_block_map().insert(
            self.last_async_call_id().get(),
            self.blockchain().get_block_nonce(),
        );

        self.delegate_contract(delegate_address.clone())
            .delegate()
            .with_gas_limit(gas_for_async_call)
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
        // for async call check
        self.async_call_start_block_map().remove(&self.last_async_call_id().get());

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.admin_delegate_success_event(caller, delegate_address, delegated_amount, self.blockchain().get_block_timestamp());
            },
            ManagedAsyncCallResult::Err(err) => {
                // restore prestaked_egld pool
                self.prestaked_egld_amount().update(|v| *v += delegated_amount);

                self.admin_delegate_fail_event(caller, delegate_address, delegated_amount, self.blockchain().get_block_timestamp(), &err.err_msg);
            },
        }
    }

    #[endpoint(adminUndelegate)]
    fn admin_undelegate(
        &self,
        undelegate_address_opt: OptionalValue<ManagedAddress>,
        opt_amount: OptionalValue<BigUint>,
    ) {
        self.require_is_owner_or_admin();
        self.require_admin_action_allowed();
        self.require_initial_configuration_is_done();

        // use auto_undelegate_address if undelegate_address_opt is None
        let undelegate_address = match undelegate_address_opt {
            OptionalValue::Some(v) => v,
            OptionalValue::None => {
                require!(
                    !self.auto_undelegate_address().is_empty(),
                    "auto_undelegate_address is empty"
                );
                self.auto_undelegate_address().get()
            }
        };

        // if amount is not given, undelegate total preunstaked amount
        let undelegating_amount = match opt_amount {
            OptionalValue::Some(v) => min(v, self.preunstaked_egld_amount().get()),
            OptionalValue::None => self.preunstaked_egld_amount().get(),
        };

        self._undelegate(undelegate_address, undelegating_amount);
    }

    #[inline]
    fn _undelegate(
        &self,
        undelegate_address: ManagedAddress,
        undelegating_amount: BigUint,
    ) {
        require!(
            undelegating_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "undelegating_amount cannot be less than 1 EGLD."
        );

        //
        self.preunstaked_egld_amount().update(|v| *v -= &undelegating_amount);

        let caller = self.blockchain().get_caller();
        let gas_for_async_call = self.get_gas_for_async_call();

        // for async call check
        self.last_async_call_id().update(|v| *v += 1);
        self.async_call_start_block_map().insert(
            self.last_async_call_id().get(),
            self.blockchain().get_block_nonce(),
        );

        self.delegate_contract(undelegate_address.clone())
            .unDelegate(undelegating_amount.clone())
            .with_gas_limit(gas_for_async_call)
            .async_call()
            .with_callback(self.callbacks().admin_undelegate_callback(
                &caller,
                &undelegate_address,
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
        // for async call check
        self.async_call_start_block_map().remove(&self.last_async_call_id().get());

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.admin_undelegate_success_event(caller, delegate_address, undelegated_amount, self.blockchain().get_block_timestamp());
            },
            ManagedAsyncCallResult::Err(err) => {
                // restore
                self.preunstaked_egld_amount().update(|v| *v += undelegated_amount);

                self.admin_undelegate_fail_event(caller, delegate_address, undelegated_amount, self.blockchain().get_block_timestamp(), &err.err_msg);
            },
        } 
    }

    ///
    #[endpoint(adminWithdraw)]
    fn admin_withdraw(&self, delegate_address: ManagedAddress) {
        self.require_is_owner_or_admin();
        self.require_admin_action_allowed();
        self.require_initial_configuration_is_done();

        let caller = self.blockchain().get_caller();
        let gas_for_async_call = self.get_gas_for_async_call();

        // for async call check
        self.last_async_call_id().update(|v| *v += 1);
        self.async_call_start_block_map().insert(
            self.last_async_call_id().get(),
            self.blockchain().get_block_nonce(),
        );

        self.delegate_contract(delegate_address.clone())
            .withdraw()
            .with_gas_limit(gas_for_async_call)
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
        // for async call check
        self.async_call_start_block_map().remove(&self.last_async_call_id().get());

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let received_egld_amount = self.call_value().egld_value();
                self.unbonded_egld_amount().update(|v| *v += &received_egld_amount);

                self.admin_withdraw_success_event(caller, delegate_address, &received_egld_amount, self.blockchain().get_block_timestamp());
            },
            ManagedAsyncCallResult::Err(_) => {
                self.admin_withdraw_fail_event(caller, delegate_address, self.blockchain().get_block_timestamp());
            },
        }
    }

    //
    #[endpoint(adminClaimRewards)]
    fn admin_claim_rewards(&self, delegate_address: ManagedAddress) {
        self.require_is_owner_or_admin();
        self.require_admin_action_allowed();
        self.require_initial_configuration_is_done();

        let caller = self.blockchain().get_caller();
        let gas_for_async_call = self.get_gas_for_async_call();

        // for async call check
        self.last_async_call_id().update(|v| *v += 1);
        self.async_call_start_block_map().insert(
            self.last_async_call_id().get(),
            self.blockchain().get_block_nonce(),
        );

        self.delegate_contract(delegate_address.clone())
            .claimRewards()
            .with_gas_limit(gas_for_async_call)
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
        // for async call check
        self.async_call_start_block_map().remove(&self.last_async_call_id().get());

        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let received_egld_amount = self.call_value().egld_value();

                let fee_egld = &received_egld_amount * self.fee().get() / MAX_PERCENTAGE;                
                let remain_egld = &received_egld_amount - &fee_egld;

                // send fee to the treasury
                if fee_egld != BigUint::zero() {
                    self.send().direct_egld(&self.treasury_wallet().get(), &fee_egld);
                }

                // add remain_egld to EGLD pool to increase vEGLD/EGLD index
                // update Prestake Pool
                self.prestaked_egld_amount().update(|v| *v += &remain_egld);
                // update LP Share Pool
                self.pool_egld_amount().update(|v| *v += &remain_egld);

                self.emit_admin_claim_rewards_success_event(
                    caller,
                    delegate_address,
                    &received_egld_amount,
                    &fee_egld,
                    self.blockchain().get_block_timestamp()
                );
            },
            ManagedAsyncCallResult::Err(_) => {
                self.emit_admin_claim_rewards_fail_event(
                    caller,
                    delegate_address,
                    self.blockchain().get_block_timestamp()
                );
            },
        }
    }

    #[inline]
    fn get_gas_for_async_call(&self) -> u64 {
        let gas_left = self.blockchain().get_gas_left();
        require!(
            gas_left > MIN_GAS_FOR_ASYNC_CALL + MIN_GAS_FOR_CALLBACK,
            ERROR_INSUFFICIENT_GAS
        );
        gas_left - MIN_GAS_FOR_CALLBACK
    }

    // if async call is not removed from async_call_start_block_map 10 blocks (MAX_BLOCKS_FOR_ASYNC_CALLBACK) after it started, it is assumed that the async call failed
    #[view(viewFailedAsyncCallIds)]
    fn view_failed_async_call_ids(&self) -> ManagedVec<usize> {
        let mut ids = ManagedVec::new();
        let current_block = self.blockchain().get_block_nonce();
        for (async_call_id, async_call_start_block) in self.async_call_start_block_map().iter() {
            if current_block > async_call_start_block + MAX_BLOCKS_FOR_ASYNC_CALLBACK {
                ids.push(async_call_id);
            }
        }
        ids
    }

    // admin will use this function for confirmed async calls
    #[only_owner]
    #[endpoint(removeFailedAsyncCallIds)]
    fn remove_failed_async_call_ids(&self, ids: MultiValueEncoded<usize>) {
        let mut async_call_start_block_map = self.async_call_start_block_map();
        for id in ids.into_iter() {
            async_call_start_block_map.remove(&id);
        }
    }
}