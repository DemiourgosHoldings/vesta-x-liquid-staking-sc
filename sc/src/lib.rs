#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod storages;
pub use storages::common_storage;
pub use storages::pool_storage;
pub mod liquid_staking;
pub use liquid_staking::management;
pub use liquid_staking::user;

pub mod error;
pub mod event;
pub mod brand;
pub mod constant;
pub mod delegate_proxy;
pub mod amm;
pub mod context;
pub mod validation;
pub mod view;
pub mod config;

#[multiversx_sc::contract]
pub trait VestaXLiquidStaking:
    common_storage::CommonStorageModule
    + pool_storage::PoolStorageModule

    + management::ManagementModule
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
