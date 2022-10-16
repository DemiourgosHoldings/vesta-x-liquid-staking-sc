elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait StorageModule
{
    // Valar
    #[view(getValarIdentifier)]
    #[storage_mapper("valar_identifier")]
    fn valar_identifier(&self) -> FungibleTokenMapper<Self::Api>;

    #[view(getValarSupply)]
    #[storage_mapper("valar_supply")]
    fn valar_supply(&self) -> SingleValueMapper<BigUint>;
}