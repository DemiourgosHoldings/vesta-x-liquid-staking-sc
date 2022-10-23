elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate_proxy;
use crate::config::{ DELEGATE_MIN_AMOUNT };

#[elrond_wasm::module]
pub trait StakeModule:
    crate::storage::common_storage::StorageModule
    + crate::storage::pool_storage::PoolModule
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
            "You must stake more than 1 EGLD at once."
        );

        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();

        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        //
        self.prestaked_egld_amount().update(|v| *v += &staking_egld_amount);
        self.prestaked_egld_amount_map().insert(&caller, self.prestaked_egld_amount_map().get(&caller).unwrap_or_default() + &staking_egld_amount);

        //
        self.delegate_contract(delegate_address.clone())
            .delegate()
            .with_egld_transfer(staking_egld_amount.clone())
            .async_call()
            .with_callback(self.callbacks().delegate_callback(&caller, &delegate_address, &staking_egld_amount))
            .call_and_exit();
    }

    #[callback]
    fn delegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        payment_amount: &BigUint,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // VALAR supply should be increased only after successful Delegation
                let staked_valar_amount = self.staked_valar_amount().get();
                let staked_egld_amount = self.staked_egld_amount().get();

                let valar_mint_amount = if staked_valar_amount == BigUint::zero() { // First Mint
                    egld_amount.clone()
                } else {
                    require!(
                        staked_egld_amount != BigUint::zero(),
                        "staked_egld_amount is zero while staked_valar_amount is not zero."
                    );

                    self.quote_valar(egld_amount)
                };

                //
                self.valar_identifier().mint_and_send(caller, valar_mint_amount.clone());

                //
                self.staked_valar_amount().set(staked_valar_amount + &valar_mint_amount);
                self.staked_egld_amount().set(staked_egld_amount + egld_amount);

                self.delegate_success_event(caller, delegate_address, &valar_mint_amount, payment_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.delegate_fail_event();
            },
        }
    }
}