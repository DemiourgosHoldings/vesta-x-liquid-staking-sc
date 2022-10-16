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
                self.burn_valar(caller, unstaking_valar_amount, unstaking_egld_amount);

                self.undelegate_success_event(caller, delegate_address, unstaking_valar_amount, unstaking_egld_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                self.undelegate_fail_event();
            },
        } 
    }

    //
    #[endpoint]
    fn claim(&self) {
       let caller = self.blockchain().get_caller();

       let mut unbonded_egld_amount_per_user = self.unbonded_egld_amount_per_user();
       let egld_amount = unbonded_egld_amount_per_user.get(&caller).unwrap_or_default();

       require!(
            egld_amount != BigUint::zero(),
            "No claimable EGLD."
       );
       require!(
            self.blockchain().get_balance(&self.blockchain().get_sc_address()) >= egld_amount,
            "No enough EGLD balance in Smart Contract."
        );

       unbonded_egld_amount_per_user.remove(&caller);

       self.send().direct_egld(&caller, &egld_amount);

       self.claim_event(&caller, &egld_amount);
    }

    //
    #[inline]
    fn mint_and_send_valar(&self, to: &ManagedAddress, egld_amount: &BigUint) -> BigUint {
        let valar_supply = self.valar_supply().get();
        let staked_egld_amount = self.staked_egld_amount().get();

        let valar_mint_amount = if valar_supply == BigUint::zero() { // First Mint
            egld_amount.clone()
        } else {
            require!(
                staked_egld_amount != BigUint::zero(),
                "staked_egld_amount is zero while valar_supply is not zero."
            );

            self.quote_valar(egld_amount)
        };

        //
        self.valar_identifier().mint_and_send(to, valar_mint_amount.clone());

        //
        self.valar_supply().set(valar_supply + &valar_mint_amount);
        self.staked_egld_amount().set(staked_egld_amount + egld_amount);

        valar_mint_amount
    }

    #[inline]
    fn burn_valar(&self, to: &ManagedAddress, valar_amount: &BigUint, egld_amount: &BigUint) {
        self.send().esdt_local_burn(&self.valar_identifier().get_token_id(), 0, valar_amount);
        self.valar_supply().update(|v| *v -= valar_amount);
        self.staked_egld_amount().update(|v| *v -= egld_amount);

        self.unbonding_egld_amount().update(|v| *v += egld_amount);

        self.unbonding_users().insert(to.clone());
        let current_timestamp = self.blockchain().get_block_timestamp();
        let mut unbonding_egld_amount_per_user = self.unbonding_egld_amount_per_user(to);
        unbonding_egld_amount_per_user.insert(current_timestamp, unbonding_egld_amount_per_user.get(&current_timestamp).unwrap_or_default() + egld_amount);
    }
}