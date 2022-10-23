elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate_proxy;
use crate::config::{ DELEGATE_MIN_AMOUNT };

#[elrond_wasm::module]
pub trait UnstakeModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::event::EventModule
    + crate::pool::PoolModule
{
    #[proxy]
    fn delegate_contract(&self, sc_address: ManagedAddress) -> delegate_proxy::Proxy<Self::Api>;

    //
    // #[payable("*")]
    // #[endpoint]
    // fn unstake(&self) {
    //    let (payment_token, _, unstaking_valar_amount) = self.call_value().single_esdt().into_tuple();

    //     require!(
    //         payment_token == self.valar_identifier().get_token_id(),
    //         "You sent a wrong token."
    //     );
    //     require!(
    //         unstaking_valar_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
    //         "You must unstake more than 1 EGLD at once."
    //     );

    //     let caller = self.blockchain().get_caller();
    //     let delegate_address = self.delegate_address().get();
    //     let unstaking_egld_amount = self.quote_egld(&unstaking_valar_amount);

    //     self.delegate_contract(delegate_address.clone())
    //         .unDelegate(unstaking_egld_amount.clone())
    //         .async_call()
    //         .with_callback(self.callbacks().undelegate_callback(&caller, &delegate_address, &unstaking_valar_amount, &unstaking_egld_amount))
    //         .call_and_exit();
    // }

    // #[callback]
    // fn undelegate_callback(
    //     &self,
    //     #[call_result] result: ManagedAsyncCallResult<()>,
    //     caller: &ManagedAddress,
    //     delegate_address: &ManagedAddress,
    //     unstaking_valar_amount: &BigUint,
    //     unstaking_egld_amount: &BigUint
    // ){
    //     match result {
    //         ManagedAsyncCallResult::Ok(()) => {
    //             // VALAR supply should be decreased only after successful Undelegation
    //             self.send().esdt_local_burn(&self.valar_identifier().get_token_id(), 0, unstaking_valar_amount);
    //             self.staked_valar_amount().update(|v| *v -= unstaking_valar_amount);
    //             self.staked_egld_amount().update(|v| *v -= unstaking_egld_amount);

    //             self.unbonding_egld_amount().update(|v| *v += unstaking_egld_amount);

    //             self.unbonding_users().insert(caller.clone());
    //             let current_timestamp = self.blockchain().get_block_timestamp();
    //             let mut unbonding_egld_amount_per_user = self.unbonding_egld_amount_per_user(caller);
    //             unbonding_egld_amount_per_user.insert(current_timestamp, unbonding_egld_amount_per_user.get(&current_timestamp).unwrap_or_default() + unstaking_egld_amount);

    //             self.undelegate_success_event(caller, delegate_address, unstaking_valar_amount, unstaking_egld_amount);
    //         },
    //         ManagedAsyncCallResult::Err(_) => {
    //             self.undelegate_fail_event();
    //         },
    //     } 
    // }

    // //
    // #[endpoint]
    // fn claim(&self) {
    //     let caller = self.blockchain().get_caller();
        
    //     let mut unbonded_egld_amount_per_user = self.unbonded_egld_amount_per_user();
    //     let egld_amount = unbonded_egld_amount_per_user.get(&caller).unwrap_or_default();
        
    //     require!(
    //         egld_amount != BigUint::zero(),
    //         "No claimable EGLD."
    //     );
    //     require!(
    //         self.blockchain().get_balance(&self.blockchain().get_sc_address()) >= egld_amount,
    //         "No enough EGLD balance in Smart Contract."
    //     );
        
    //     unbonded_egld_amount_per_user.remove(&caller);
        
    //     self.send().direct_egld(&caller, &egld_amount);
        
    //     self.claim_event(&caller, &egld_amount);
    // }
}