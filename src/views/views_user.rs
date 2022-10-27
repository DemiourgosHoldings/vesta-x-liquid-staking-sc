elrond_wasm::imports!();
elrond_wasm::derive_imports!();

// use crate::state::{ UnstakingPack };

#[elrond_wasm::module]
pub trait ViewUserModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::amm::AmmModule
{
}