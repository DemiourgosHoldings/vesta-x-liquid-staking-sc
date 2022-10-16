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
                let minted_valar_amount = self.mint_and_send_valar(caller, payment_amount);

                self.delegate_success_event(caller, delegate_address, &minted_valar_amount, payment_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.delegate_fail_event();
            },
        }
    }

    //
    #[payable("*")]
    #[endpoint]
    fn unstake(&self) {
       let (payment_token, _, unstaking_valar_amount) = self.call_value().single_esdt().into_tuple();

        require!(
            payment_token == self.valar_identifier().get_token_id(),
            "You sent a wrong token."
        );
        require!(
            unstaking_valar_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "You must unstake more than 1 EGLD at once."
        );

        let caller = self.blockchain().get_caller();
        let delegate_address = self.delegate_address().get();
        let unstaking_egld_amount = self.quote_egld(&unstaking_valar_amount);

        self.delegate_contract(delegate_address.clone())
            .unDelegate(unstaking_egld_amount.clone())
            .async_call()
            .with_callback(self.callbacks().undelegate_callback(&caller, &delegate_address, &unstaking_valar_amount, &unstaking_egld_amount))
            .call_and_exit();
    }

    #[callback]
    fn undelegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        unstaking_valar_amount: &BigUint,
        unstaking_egld_amount: &BigUint
    ){
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // VALAR supply should be decreased only after successful Undelegation
                self.burn_valar(unstaking_valar_amount, unstaking_egld_amount);

                self.undelegate_success_event(caller, delegate_address, unstaking_valar_amount, unstaking_egld_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.undelegate_fail_event();
            },
    } }
}