multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AmmModule:
    crate::pool_storage::PoolStorageModule
{
    #[view(quoteVegld)]
    fn quote_vegld(&self, egld_amount: &BigUint) -> BigUint {
        require!(
            self.pool_egld_amount().get() != BigUint::zero(),
            "pool_egld_amount is zero"
        );
        self.pool_vegld_amount().get() * egld_amount / &self.pool_egld_amount().get()
    }

    #[view(quoteEgld)]
    fn quote_egld(&self, vegld_amount: &BigUint) -> BigUint {
        require!(
            self.pool_vegld_amount().get() != BigUint::zero(),
            "pool_vegld_amount is zero"
        );
        self.pool_egld_amount().get() * vegld_amount / &self.pool_vegld_amount().get()
    }
}