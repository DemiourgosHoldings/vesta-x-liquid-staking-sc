elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::delegate_proxy;
use crate::config::{ DELEGATE_MIN_AMOUNT };
use crate::state::{ UserActionItem };

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
        
        self.preunstaked_valar_amount().update(|v| *v += &unstaking_valar_amount);
        self.preunstaked_valar_amount_queue().push_back(
            UserActionItem {
                address: caller.clone(),
                amount: unstaking_valar_amount.clone(),
                timestamp: self.blockchain().get_block_timestamp(),
            }
        );

        self.user_preunstake_event(&caller, &unstaking_valar_amount);
    }

    #[only_owner]
    #[endpoint(adminUndelegateFixedAmount)]
    fn admin_undelegate_fixed_amount(
        &self,
        delegate_address: ManagedAddress,
        target_undelegate_egld_amount: BigUint,  // EGLD
    ) {
        require!(
            target_undelegate_egld_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "target_undelegate_egld_amount cannot be less than 1 EGLD."
        );

        let caller = self.blockchain().get_caller();
        let target_burn_valar_amount = self.quote_valar(&target_undelegate_egld_amount);

        // make a list of delegating users and amounts within target_delegate_amount
        let mut real_burn_valar_amount = BigUint::zero();
        let mut undelegating_item_count = 0;
        let mut last_item_amount = BigUint::zero();
        let preunstaked_valar_amount_queue = self.preunstaked_valar_amount_queue();
        for item in preunstaked_valar_amount_queue.iter() {
            let preunstaked_item = item.into_value();
            let final_amount = if real_burn_valar_amount.clone() + &preunstaked_item.amount > target_burn_valar_amount {
                if real_burn_valar_amount == target_burn_valar_amount {
                    BigUint::zero()
                } else {
                    target_burn_valar_amount.clone() - &real_burn_valar_amount
                }
            } else {
                preunstaked_item.amount.clone()
            };
            // if target_burn_valar_amount is reached, break the loop
            if final_amount == BigUint::zero() && preunstaked_item.amount != BigUint::zero() {
                break;
            }

            real_burn_valar_amount += &final_amount;
            undelegating_item_count += 1;

            // target_burn_valar_amount is reached and the last item should be splited
            if final_amount < preunstaked_item.amount {
                last_item_amount = final_amount;
                break;
            }
        }

        let real_undelegating_egld_amount = self.quote_egld(&real_burn_valar_amount);
        require!(
            real_undelegating_egld_amount >= BigUint::from(DELEGATE_MIN_AMOUNT),
            "real_undelegating_egld_amount cannot be less than 1 EGLD."
        );

        self.delegate_contract(delegate_address.clone())
            .unDelegate(real_undelegating_egld_amount.clone())
            .async_call()
            .with_callback(self.callbacks().admin_undelegate_callback(
                &caller,
                &delegate_address,
                &real_burn_valar_amount,
                &real_undelegating_egld_amount,
                undelegating_item_count,
                &last_item_amount,
            ))
            .call_and_exit();
    }

    #[callback]
    fn admin_undelegate_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<()>,
        caller: &ManagedAddress,
        delegate_address: &ManagedAddress,
        real_burn_valar_amount: &BigUint,
        real_undelegating_egld_amount: &BigUint,
        undelegating_item_count: usize,
        last_item_amount: &BigUint,
    ){
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                // VALAR supply should be decreased only after successful Undelegation
                self.send().esdt_local_burn(&self.valar_identifier().get_token_id(), 0, real_burn_valar_amount);

                // update LP Share Pool
                self.pool_valar_amount().update(|v| *v -= real_burn_valar_amount);
                self.pool_egld_amount().update(|v| *v -= real_undelegating_egld_amount);

                // update User Account
                let mut preunstaked_valar_amount_queue = self.preunstaked_valar_amount_queue();
                for _ in 0..(undelegating_item_count - 1) {
                    preunstaked_valar_amount_queue.pop_front();
                }
                if *last_item_amount == BigUint::zero() { 
                    preunstaked_valar_amount_queue.pop_front();
                } else { // if the last time is splitted
                    match preunstaked_valar_amount_queue.front() {
                        Some(item) => {
                            let item_ref = item.get_value_as_ref();
                            let new_item = UserActionItem {
                                address: item_ref.address.clone(),
                                amount: item_ref.amount.clone() - last_item_amount,
                                timestamp: item_ref.timestamp,
                            };
                            preunstaked_valar_amount_queue.set_node_value(item, new_item); // Note: must check changed amount is saved in LinkedListMapper
                        },
                        None => sc_panic!("undelegating_item_count and the length of preunstaked_valar_amount_queue not match.")
                    }
                }

                //
                self.preunstaked_valar_amount().update(|v| *v -= real_burn_valar_amount);

                self.admin_undelegate_success_event(caller, delegate_address, real_burn_valar_amount, real_undelegating_egld_amount);
            },
            ManagedAsyncCallResult::Err(err) => {
                self.admin_undelegate_fail_event(caller, delegate_address, real_burn_valar_amount, real_undelegating_egld_amount, &err.err_msg);
            },
        } 
    }

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