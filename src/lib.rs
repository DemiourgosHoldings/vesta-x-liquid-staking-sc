#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub mod storage;
pub mod stake;
pub mod event;
pub mod valar;
pub mod config;

#[elrond_wasm::contract]
pub trait ValarLiquidStaking:
    storage::StorageModule
    + event::EventModule
    + stake::StakeModule
    + valar::ValarModule
{
    #[init]
    fn init(&self) {
    }
}
