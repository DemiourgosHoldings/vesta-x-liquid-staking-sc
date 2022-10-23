#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod common_storage;
pub mod stake;
pub mod event;
pub mod valar;
pub mod config;
pub mod delegate_proxy;
pub mod pool;
pub mod admin;

#[elrond_wasm::contract]
pub trait ValarLiquidStaking:
    common_storage::StorageModule
    + event::EventModule
    + stake::StakeModule
    + valar::ValarModule
    + pool::PoolModule
    + admin::AdminModule
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
