elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait StakeModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
}