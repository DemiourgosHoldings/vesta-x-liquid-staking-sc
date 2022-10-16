elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait StorageModule
{
    #[view(getValarIdentifier)]
    #[storage_mapper("valar_identifier")]
    fn valar_identifier(&self) -> FungibleTokenMapper<Self::Api>;
}