elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::constant::{ MAX_PERCENTAGE };

#[elrond_wasm::module]
pub trait ConfigModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::event::EventModule
    + crate::validation::ValidationModule
{    
    #[only_owner]
    #[endpoint(setSettings)]
    fn set_settings(
        &self,
        unbonding_period: u64,
        treasury_wallet: ManagedAddress,
        fee: u64,
    ) {
        self.unbonding_period().set(unbonding_period);
        self.set_treasury_wallet(treasury_wallet);
        self.set_fee(fee);
        self.user_action_allowed().set(true);
        self.admin_action_allowed().set(true);
    }

    #[only_owner]
    #[endpoint(setTreasuryWallet)]
    fn set_treasury_wallet(
        &self,
        treasury_wallet: ManagedAddress,
    ) {
        self.treasury_wallet().set(&treasury_wallet);
        
        self.change_treasury_wallet_event(&self.blockchain().get_caller(), &treasury_wallet, self.blockchain().get_block_timestamp());
    }

    #[only_owner]
    #[endpoint(setFee)]
    fn set_fee(
        &self,
        fee: u64,
    ) {
        require!(
            fee <= MAX_PERCENTAGE,
            "fee cannot be higher than 100%."
        );
        self.fee().set(fee);

        self.change_fee_event(&self.blockchain().get_caller(), fee, self.blockchain().get_block_timestamp());
    }

    #[only_owner]
    #[endpoint(setUnbondingPeriod)]
    fn set_unbonding_period(
        &self,
        unbonding_period: u64,
    ) {
        self.unbonding_period().set(unbonding_period);
    }

    ///
    #[only_owner]
    #[endpoint(addAdmins)]
    fn add_admins(
        &self,
        addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        let mut admins = self.admins();
        for address in addresses {
            admins.insert(address);
        }
    }

    #[only_owner]
    #[endpoint(removeAdmins)]
    fn remove_admins(
        &self,
        addresses: MultiValueEncoded<ManagedAddress>,
    ) {
        let mut admins = self.admins();
        for address in addresses {
            admins.swap_remove(&address);
        }
    }

    ///
    #[endpoint(setUserActionAllowed)]
    fn set_user_action_allowed(
        &self,
        user_action_allowed: bool,
    ) {
        self.require_is_owner_or_admin();

        self.user_action_allowed().set(user_action_allowed);
    }

    #[only_owner]
    #[endpoint(setAdminActionAllowed)]
    fn set_admin_action_allowed(
        &self,
        admin_action_allowed: bool,
    ) {
        self.admin_action_allowed().set(admin_action_allowed);
    }

    //
    #[endpoint(setAutoDelegateAddress)]
    fn set_auto_delegate_address(
        &self,
        auto_delegate_address: ManagedAddress,
    ) {
        self.require_is_owner_or_admin();

        self.auto_delegate_address().set(&auto_delegate_address);
    }

    #[endpoint(removeAutoDelegateAddress)]
    fn remove_auto_delegate_address(
        &self,
    ) {
        self.require_is_owner_or_admin();

        self.auto_delegate_address().clear();
    }

    #[endpoint(setAutoUndelegateAddress)]
    fn set_auto_undelegate_address(
        &self,
        auto_undelegate_address: ManagedAddress,
    ) {
        self.require_is_owner_or_admin();

        self.auto_undelegate_address().set(&auto_undelegate_address);
    }

    #[endpoint(removeAutoUndelegateAddress)]
    fn remove_auto_undelegate_address(
        &self,
    ) {
        self.require_is_owner_or_admin();

        self.auto_undelegate_address().clear();
    }
}