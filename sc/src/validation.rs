elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::constant::*;

#[elrond_wasm::module]
pub trait ValidationModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
{    
    #[view(isOwnerOrAdmin)]
    fn is_owner_or_admin(&self, address: &ManagedAddress) -> bool {
        *address == self.blockchain().get_owner_address() || self.admins().contains(address)
    }

    #[inline]
    fn require_is_owner_or_admin(&self) {
        require!(
            self.is_owner_or_admin(&self.blockchain().get_caller()),
            "You are not Owner or Admin."
        );
    }

    #[inline]
    fn require_user_action_allowed(&self) {
        require!(
            self.user_action_allowed().get(),
            "User Action is not allowed."
        );
    }

    #[inline]
    fn require_management_action_allowed(&self) {
        require!(
            self.management_action_allowed().get(),
            "Management Action is not allowed."
        );
    }

    #[inline]
    fn require_initial_configuration_is_done(&self) {
        require!(
            !self.treasury_wallet().is_empty() && !self.treasury_wallet().get().is_zero(),
            "treasury_wallet is not set"
        );
        let unbonding_period = self.unbonding_period().get();
        require!(
            MIN_UNBONDING_PERIOD <= unbonding_period && unbonding_period <= MAX_UNBONDING_PERIOD,
            "unbonding_period must be in range of {} and {}",
            MIN_UNBONDING_PERIOD, MAX_UNBONDING_PERIOD
        );

        let roles = self.blockchain().get_esdt_local_roles(self.vegld_identifier().get_token_id_ref());
        require!(
            roles.has_role(&EsdtLocalRole::Mint),
            "Smart Contract does not have Mint role of vEGLD token"
        );
        require!(
            roles.has_role(&EsdtLocalRole::Burn),
            "Smart Contract does not have Burn role of vEGLD token"
        );
    }

    #[view(isTokenRolesSet)]
    fn is_token_roles_set(&self) -> bool {
        let roles = self.blockchain().get_esdt_local_roles(self.vegld_identifier().get_token_id_ref());
        roles.has_role(&EsdtLocalRole::Mint) && roles.has_role(&EsdtLocalRole::Burn)
    }

    #[inline]
    fn require_whitelisted_staking_provider(&self, address: &ManagedAddress) {
        require!(
            self.whitelisted_sp_addresses().contains(address),
            "Given Staking Provider is not whitelisted"
        );
    }

    #[inline]
    fn require_is_address_smart_contract_and_on_metachain(&self, address: &ManagedAddress) {
        require!(
            self.blockchain().is_smart_contract(address),
            "Given address is not smart contract"
        );
        require!(
            self.blockchain().get_shard_of_address(address) == METACHAIN_SHARD_ID,
            "Given address is on Metachain"
        );
    }
}