elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate;
use crate::config::{ DELEGATE_MIN_AMOUNT };

#[elrond_wasm::module]
pub trait StakeModule:
    crate::storage::StorageModule
    + crate::event::EventModule
    + crate::pool::PoolModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate::Proxy<Self::Api>;

    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        let payment_amount = self.call_value().egld_value();

        require!(
            payment_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "You must stake more than 1 EGLD."
        );

        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();

        require!(
            !delegate_address.is_zero(),
            "delegate_address is not set yet."
        );

        self.delegate_contract(delegate_address.clone())
            .delegate()
            .with_egld_transfer(payment_amount.clone())
            .async_call()
            .with_callback(self.callbacks().delegate_callback(&caller, &delegate_address, &payment_amount))
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
                let minted_valar_amount = self.mint_valar(payment_amount);
                self.send().direct_esdt(caller, &self.valar_identifier().get_token_id(), 0, &minted_valar_amount);
                self.stake_success_event(caller, delegate_address, payment_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.stake_fail_event();
            },
        }
    }
}