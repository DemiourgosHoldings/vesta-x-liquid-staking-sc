elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait CommonStorageModule
{
    // Valar Token
    #[view(getValarIdentifier)]
    #[storage_mapper("valar_identifier")]
    fn valar_identifier(&self) -> FungibleTokenMapper<Self::Api>;

    //
    #[view(getTreasuryWallet)]
    #[storage_mapper("treasury_wallet")]
    fn treasury_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    //
    #[view(getUnbondingPeriod)]
    #[storage_mapper("unbonding_period")]
    fn unbonding_period(&self) -> SingleValueMapper<u64>;
}