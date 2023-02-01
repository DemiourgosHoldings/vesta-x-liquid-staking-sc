elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait ValidationModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
{    
    #[inline]
    fn require_is_owner_or_admin(&self) {
        let caller = self.blockchain().get_caller();
        require!(
            caller == self.blockchain().get_owner_address() || self.admins().contains(&caller),
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
    fn require_admin_action_allowed(&self) {
        require!(
            self.admin_action_allowed().get(),
            "Admin Action is not allowed."
        );
    }

    #[inline]
    fn require_initial_configuration_is_done(&self) {
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
}