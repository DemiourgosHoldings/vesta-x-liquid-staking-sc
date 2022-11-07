elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{ UnstakingPack };

#[elrond_wasm::module]
pub trait UserModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::event::EventModule
    + crate::amm::AmmModule
    + crate::validation::ValidationModule
{
    #[payable("EGLD")]
    #[endpoint]
    fn stake(&self) {
        self.require_user_action_allowed();
        self.require_initial_configuration_is_done();

        let staking_egld_amount = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        // VALAR supply should be increased only after successful Delegation
        let pool_valar_amount = self.pool_valar_amount().get();
        let pool_egld_amount = self.pool_egld_amount().get();

        let valar_mint_amount = if pool_valar_amount == BigUint::zero() {
            // when LP Share Pool is empty, mint the same amount of VALAR as EGLD amount
            // VALAR : EGLD = 1 : 1
            staking_egld_amount.clone()
        } else {
            require!(
                pool_egld_amount != BigUint::zero(),
                "staked_egld_amount is zero while staked_valar_amount is not zero."
            );

            // VALAR : EGLD = pool_valar_amount : pool_egld_amount
            self.quote_valar(&staking_egld_amount)
        };

        // update Prestake Pool
        self.prestaked_egld_amount().update(|v| *v += &staking_egld_amount);

        // update LP Share Pool
        self.pool_valar_amount().set(pool_valar_amount + &valar_mint_amount);
        self.pool_egld_amount().set(pool_egld_amount + &staking_egld_amount);

        // mint VALAR and send it to the caller
        self.valar_identifier().mint_and_send(&caller, valar_mint_amount.clone());

        //
        self.user_stake_event(&caller, &staking_egld_amount, &valar_mint_amount);
    }

    //
    #[payable("*")]
    #[endpoint]
    fn unstake(&self) {
        self.require_user_action_allowed();
        self.require_initial_configuration_is_done();

        let (payment_token, _, unstaking_valar_amount) = self.call_value().single_esdt().into_tuple();

        require!(
            payment_token == self.valar_identifier().get_token_id(),
            "You sent wrong token."
        );

        let caller = self.blockchain().get_caller();
        
        // burn VALAR
        self.send().esdt_local_burn(&self.valar_identifier().get_token_id(), 0, &unstaking_valar_amount);
        let unstaking_egld_amount = self.quote_egld(&unstaking_valar_amount);
    
        self.pool_valar_amount().update(|v| *v -= &unstaking_valar_amount);
        self.pool_egld_amount().update(|v| *v -= &unstaking_egld_amount);
        self.preunstaked_egld_amount().update(|v| *v += &unstaking_egld_amount);

        self.unstaking_users().insert(caller.clone());
        self.unstaking_packs(&caller).push_back(UnstakingPack {
            amount: unstaking_egld_amount.clone(),
            timestamp: self.blockchain().get_block_timestamp(),
        });

        self.user_unstake_event(&caller, &unstaking_valar_amount, &unstaking_egld_amount);
    }

    //
    #[endpoint]
    fn withdraw(&self) {
        self.require_user_action_allowed();
        self.require_initial_configuration_is_done();

        let caller = self.blockchain().get_caller();
        let current_timestamp = self.blockchain().get_block_timestamp();
        let unbonding_period = self.unbonding_period().get();

        let mut unbonded_amount = BigUint::zero();
        let mut unbonded_count = 0;
        let mut unstaking_packs = self.unstaking_packs(&caller);

        // check how many packs are withdrawable
        // a pack is withdrawable only if unbonding_period of the pack is passed
        for node in unstaking_packs.iter() {
            let item = node.get_value_as_ref();
            if current_timestamp >= item.timestamp + unbonding_period {
                unbonded_amount += &item.amount;
                unbonded_count += 1;
            } else {
                break;
            }
        }

        require!(
            unbonded_amount != BigUint::zero(),
            "No EGLD to withdraw."
        );
        // Withdrawable Amount = EGLD_Balance_of_SC - Total_Prestaked_Amount
        require!(
            self.unbonded_egld_amount().get() >= unbonded_amount,
            "EGLD is not unbonded from delegate providers yet."
        );
        require!(
            unbonded_amount <= self.blockchain().get_balance(&self.blockchain().get_sc_address()),
            "Not enough EGLD in Smart Contract."
        );

        // remove withdrawable packs
        for _ in 0..unbonded_count {
            unstaking_packs.pop_front();
        }
        // if no pack is left, remove user from unstaking_users
        if unstaking_packs.len() == 0 && self.unstaking_users().contains(&caller) {
            self.unstaking_users().swap_remove(&caller);
        }
        self.unbonded_egld_amount().update(|v| *v -= &unbonded_amount);
        
        self.send().direct_egld(&caller, &unbonded_amount);
        
        self.user_withdraw_event(&caller, &unbonded_amount);
    }


    /// Put EGLD to PreUnstake Pool without minting VALAR
    #[payable("EGLD")]
    #[endpoint]
    fn donate(&self) {
        self.require_user_action_allowed();
        self.require_initial_configuration_is_done();

        let staking_egld_amount = self.call_value().egld_value();
        let caller = self.blockchain().get_caller();

        self.prestaked_egld_amount().update(|v| *v += &staking_egld_amount);
        self.pool_egld_amount().update(|v| *v += &staking_egld_amount);

        //
        self.donate_event(&caller, &staking_egld_amount);
    }
}