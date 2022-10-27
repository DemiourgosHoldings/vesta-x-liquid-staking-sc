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
mod views;
use views::views_pool;
use views::views_user;

pub mod event;
pub mod valar;
pub mod config;
pub mod delegate_proxy;
pub mod amm;
pub mod state;

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

    + views_pool::ViewPoolModule
    + views_user::ViewUserModule
{
    #[init]
    fn init(&self) {
    }

    #[only_owner]
    #[endpoint(setTreasuryWallet)]
    fn set_treasury_wallet(
        &self,
        treasury_wallet: ManagedAddress,
    ) {
        self.treasury_wallet().set(&treasury_wallet);
        self.change_treasury_wallet(&treasury_wallet);
    }
}
