elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::config::{ ONE_VALAR_IN_WEI, ONE_EGLD_IN_WEI };

#[elrond_wasm::module]
pub trait ViewPoolModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::amm::AmmModule
{
    /// return EGLD amount corresponding to 1 VALAR
    #[view(getValarPrice)]
    fn get_valar_price(&self) -> BigUint {
        self.quote_egld(&BigUint::from(ONE_VALAR_IN_WEI))
    }

    #[view(getEgldPrice)]
    fn get_egld_price(&self) -> BigUint {
        self.quote_valar(&BigUint::from(ONE_EGLD_IN_WEI))
    }
}