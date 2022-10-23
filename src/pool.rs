elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait PoolModule:
    crate::pool_storage::PoolStorageModule
    + crate::event::EventModule
{
    #[view(quoteValar)]
    fn quote_valar(&self, egld_amount: &BigUint) -> BigUint {
        self.staked_valar_amount().get() * egld_amount / &self.staked_egld_amount().get()
    }

    #[view(quoteEgld)]
    fn quote_egld(&self, valar_amount: &BigUint) -> BigUint {
        self.staked_egld_amount().get() * valar_amount / &self.staked_valar_amount().get()
    }
}