#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod storages;
use storages::common_storage;
use storages::pool_storage;
mod liquid_staking;
use liquid_staking::admin;
use liquid_staking::user;
use liquid_staking::pool;

pub mod event;
pub mod valar;
pub mod config;
pub mod delegate_proxy;
pub mod amm;
pub mod state;
pub mod validation;
pub mod view;

#[elrond_wasm::contract]
pub trait ValarLiquidStaking:
    common_storage::CommonStorageModule
    + pool_storage::PoolStorageModule

    + admin::AdminModule
    + user::UserModule
    + pool::PoolModule

    + event::EventModule
    + valar::ValarModule
    + amm::AmmModule
    + validation::ValidationModule

    + view::ViewModule
{
    #[init]
    fn init(&self) {
    }

    #[only_owner]
    #[endpoint(setSettings)]
    fn set_settings(
        &self,
        unbonding_period: u64,
        treasury_wallet: ManagedAddress,
    ) {
        self.unbonding_period().set(unbonding_period);
        self.treasury_wallet().set(&treasury_wallet);
        self.user_action_allowed().set(true);
        self.admin_action_allowed().set(true);

        self.change_treasury_wallet_event(&treasury_wallet);
    }

    #[only_owner]
    #[endpoint(setTreasuryWallet)]
    fn set_treasury_wallet(
        &self,
        treasury_wallet: ManagedAddress,
    ) {
        self.treasury_wallet().set(&treasury_wallet);
        
        self.change_treasury_wallet_event(&treasury_wallet);
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

    ///
    #[endpoint(adminTransferEgld)]
    fn admin_transfer_egld(
        &self,
        opt_amount: OptionalValue<BigUint>
    ) {
        self.require_is_owner_or_admin();

        // if token_amount is not given, set it to balance of SC - max value to withdraw
        let amount = match opt_amount {
            OptionalValue::Some(v) => v,
            OptionalValue::None => self.blockchain().get_balance(&self.blockchain().get_sc_address()),
        };

        let treasury_wallet = self.treasury_wallet().get();
        self.send().direct_egld(&treasury_wallet, &amount);

        self.admin_transfer_egld_event(&treasury_wallet, &amount);
    }
}
