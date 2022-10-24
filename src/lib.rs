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
use liquid_staking::pool;

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

    + rewards::RewardsModule
    + stake::StakeModule
    + unstake::UnstakeModule
    + pool::PoolModule

    + event::EventModule
    + valar::ValarModule
    + amm::AmmModule
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
        self.change_delegate_address(&delegate_address);
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

    #[only_owner]
    #[endpoint(setAutoDelegateEnabled)]
    fn set_auto_delegate_enabled(
        &self,
        auto_delegate_enabled: bool,
    ) {
        self.auto_delegate_enabled().set(auto_delegate_enabled);
        self.change_auto_delegate_enabled(auto_delegate_enabled);
    }
}
