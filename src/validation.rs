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
}