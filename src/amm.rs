elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait AmmModule:
    crate::pool_storage::PoolStorageModule
{
    #[view(quoteValar)]
    fn quote_valar(&self, egld_amount: &BigUint) -> BigUint {
        require!(
            self.pool_egld_amount().get() != BigUint::zero(),
            "pool_egld_amount is zero"
        );
        self.pool_valar_amount().get() * egld_amount / &self.pool_egld_amount().get()
    }

    #[view(quoteEgld)]
    fn quote_egld(&self, valar_amount: &BigUint) -> BigUint {
        require!(
            self.pool_valar_amount().get() != BigUint::zero(),
            "pool_valar_amount is zero"
        );
        self.pool_egld_amount().get() * valar_amount / &self.pool_valar_amount().get()
    }
}