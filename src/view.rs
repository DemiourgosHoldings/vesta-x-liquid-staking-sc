elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{ ONE_VALAR_IN_WEI, ONE_EGLD_IN_WEI };
use crate::state::{ LiquidStakingSettings };

#[elrond_wasm::module]
pub trait ViewModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::amm::AmmModule
{
    /// return EGLD amount corresponding to 1 VALAR
    #[view(getValarPrice)]
    fn get_valar_price(&self) -> BigUint {
        self.quote_egld(&BigUint::from(ONE_VALAR_IN_WEI))
    }

    #[view(getEgldPrice)]
    fn get_egld_price(&self) -> BigUint {
        self.quote_valar(&BigUint::from(ONE_EGLD_IN_WEI))
    }

    #[view(isOwnerOrAdmin)]
    fn is_owner_or_admin(&self, caller: ManagedAddress) -> bool {
        caller == self.blockchain().get_owner_address() || self.admins().contains(&caller)
    }

    #[view(viewLiquidStakingSettings)]
    fn view_liquid_staking_settings(&self) -> LiquidStakingSettings<Self::Api> {
        let admins_set = self.admins();
        let mut admins = ManagedVec::new();
        for admin in admins_set.iter() {
            admins.push(admin);
        }

        LiquidStakingSettings {
            valar_identifier: self.valar_identifier().get_token_id(),
            treasury_wallet: self.treasury_wallet().get(),
            fee: self.fee().get(),
            unbonding_period: self.unbonding_period().get(),
            admins,
            user_action_allowed: self.user_action_allowed().get(),
            admin_action_allowed: self.admin_action_allowed().get(),
            pool_valar_amount: self.pool_valar_amount().get(),
            pool_egld_amount: self.pool_egld_amount().get(),
            prestaked_egld_amount: self.prestaked_egld_amount().get(),
            preunstaked_egld_amount: self.preunstaked_egld_amount().get(),
            unstaking_egld_amount: self.unstaking_egld_amount().get(),
            unbonded_egld_amount: self.unbonded_egld_amount().get(),
        }
    }
}