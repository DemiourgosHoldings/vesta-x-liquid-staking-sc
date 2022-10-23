elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate_proxy;
use crate::config::{ DELEGATE_MIN_AMOUNT };

#[elrond_wasm::module]
pub trait StakeModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::event::EventModule
    + crate::pool::PoolModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate_proxy::Proxy<Self::Api>;

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let staking_egld_amount = self.call_value().egld_value();
        require!(
            staking_egld_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "Cannot stake less than 1 EGLD at once."
        );

        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();
        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        // update Prestake Pool
        self.prestaked_egld_amount().update(|v| *v += &staking_egld_amount);
        self.prestaked_egld_amount_map().insert(caller.clone(), self.prestaked_egld_amount_map().get(&caller).unwrap_or_default() + &staking_egld_amount);

        // if Auto-Delegate is enabled, EGLD will be delegated with Caller's gas fee
        if self.auto_delegate_enabled().get() {
            self.delegate_contract(delegate_address.clone())
                .delegate()
                .with_egld_transfer(staking_egld_amount.clone())
                .async_call()
                .with_callback(self.callbacks().delegate_callback(&caller, &delegate_address, &staking_egld_amount))
                .call_and_exit();
        }
        
        //
        self.prestake_event(&caller, &staking_egld_amount, self.auto_delegate_enabled().get());
    }

    #[callback]
    fn delegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        delegated_egld_amount: &BigUint,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // VALAR supply should be increased only after successful Delegation
                let pool_valar_amount = self.pool_valar_amount().get();
                let pool_egld_amount = self.pool_egld_amount().get();

                let valar_mint_amount = if pool_valar_amount == BigUint::zero() {
                    // when LP Share Pool is empty, mint the same amount of VALAR as EGLD amount
                    // VALAR : EGLD = 1 : 1
                    delegated_egld_amount.clone()
                } else {
                    require!(
                        pool_egld_amount != BigUint::zero(),
                        "staked_egld_amount is zero while staked_valar_amount is not zero."
                    );

                    // VALAR : EGLD = pool_valar_amount : pool_egld_amount
                    self.quote_valar(delegated_egld_amount)
                };

                //
                self.valar_identifier().mint_and_send(caller, valar_mint_amount.clone());

                // update Prestake Pool
                self.prestaked_egld_amount().update(|v| *v -= delegated_egld_amount);
                self.prestaked_egld_amount_map().insert(caller.clone(), self.prestaked_egld_amount_map().get(caller).unwrap_or_default() - delegated_egld_amount);

                // update LP Share Pool
                self.pool_valar_amount().set(pool_valar_amount + &valar_mint_amount);
                self.pool_egld_amount().set(pool_egld_amount + delegated_egld_amount);

                self.delegate_success_event(caller, delegate_address, &valar_mint_amount, delegated_egld_amount);
            },
            ManagedAsyncCallResult::Err(err) => {
                self.delegate_fail_event(caller, delegate_address, delegated_egld_amount, &err.err_msg);
            },
        }
    }
}