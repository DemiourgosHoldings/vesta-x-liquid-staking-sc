#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod storages;
use storages::common_storage;
use storages::pool_storage;
mod liquid_staking;
use liquid_staking::admin;
use liquid_staking::user;

pub mod event;
pub mod brand;
pub mod constant;
pub mod delegate_proxy;
pub mod amm;
pub mod context;
pub mod validation;
pub mod view;
pub mod config;

#[elrond_wasm::contract]
pub trait VestaXLiquidStaking:
    common_storage::CommonStorageModule
    + pool_storage::PoolStorageModule

    + admin::AdminModule
    + user::UserModule

    + event::EventModule
    + brand::BrandModule
    + amm::AmmModule
    + validation::ValidationModule

    + view::ViewModule
    + config::ConfigModule
{
    #[init]
    fn init(&self) {}
}
