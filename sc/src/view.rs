elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::constant::{ ONE_VEGLD_IN_WEI, ONE_EGLD_IN_WEI };
use crate::context::{ LiquidStakingSettings };

#[elrond_wasm::module]
pub trait ViewModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::amm::AmmModule
    + crate::validation::ValidationModule
{
    /// return EGLD amount corresponding to 1 VEGLD
    #[view(getVegldPrice)]
    fn get_vegld_price(&self) -> BigUint {
        self.quote_egld(&BigUint::from(ONE_VEGLD_IN_WEI))
    }

    #[view(getEgldPrice)]
    fn get_egld_price(&self) -> BigUint {
        self.quote_vegld(&BigUint::from(ONE_EGLD_IN_WEI))
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

        // to prevent panic when pool_vegld_amount is zero
        let vegld_price = if self.pool_vegld_amount().get() != BigUint::zero() {
            self.get_vegld_price()
        } else {
            BigUint::zero()
        };

        LiquidStakingSettings {
            vegld_identifier: self.vegld_identifier().get_token_id(),
            treasury_wallet: self.treasury_wallet().get(),
            fee: self.fee().get(),
            unbonding_period: self.unbonding_period().get(),
            admins,
            user_action_allowed: self.user_action_allowed().get(),
            admin_action_allowed: self.admin_action_allowed().get(),
            is_token_roles_set: self.is_token_roles_set(),

            pool_vegld_amount: self.pool_vegld_amount().get(),
            pool_egld_amount: self.pool_egld_amount().get(),
            prestaked_egld_amount: self.prestaked_egld_amount().get(),
            preunstaked_egld_amount: self.preunstaked_egld_amount().get(),
            unstaking_egld_amount: self.unstaking_egld_amount().get(),
            unbonded_egld_amount: self.unbonded_egld_amount().get(),
            vegld_price,
        }
    }
}