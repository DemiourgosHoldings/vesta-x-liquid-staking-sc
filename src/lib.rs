#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod storages;
use storages::common_storage;
use storages::pool_storage;
mod liquid_staking;
use liquid_staking::rewards;
use liquid_staking::stake;
use liquid_staking::unstake;

pub mod event;
pub mod valar;
pub mod config;
pub mod delegate_proxy;
pub mod pool;
pub mod context;

#[elrond_wasm::contract]
pub trait ValarLiquidStaking:
    common_storage::CommonStorageModule
    + pool_storage::PoolStorageModule
    + rewards::RewardsModule
    + stake::StakeModule
    + unstake::UnstakeModule

    + event::EventModule
    + valar::ValarModule
    + pool::PoolModule
{
    #[init]
    fn init(&self) {
    }

    #[only_owner]
    #[endpoint(setDelegateAddress)]
    fn set_delegate_address(
        &self,
        delegate_address: ManagedAddress,
    ) {
        self.delegate_address().set(&delegate_address);
    }

    #[only_owner]
    #[endpoint(setTreasuryWallet)]
    fn set_treasury_wallet(
        &self,
        treasury_wallet: ManagedAddress,
    ) {
        self.treasury_wallet().set(&treasury_wallet);
    }
}
