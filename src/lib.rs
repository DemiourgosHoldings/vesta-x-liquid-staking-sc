#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod storage;
pub mod stake;
pub mod event;
pub mod valar;
pub mod config;
pub mod delegate;
pub mod pool;

#[elrond_wasm::contract]
pub trait ValarLiquidStaking:
    storage::StorageModule
    + event::EventModule
    + stake::StakeModule
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
}
