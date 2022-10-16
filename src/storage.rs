elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait StorageModule
{
    // Valar
    #[view(getValarIdentifier)]
    #[storage_mapper("valar_identifier")]
    fn valar_identifier(&self) -> FungibleTokenMapper<Self::Api>;

    // Pool
    #[view(getValarSupply)]
    #[storage_mapper("valar_supply")]
    fn valar_supply(&self) -> SingleValueMapper<BigUint>;

    #[view(getStakedEgldAmount)]
    #[storage_mapper("staked_egld_amount")]
    fn staked_egld_amount(&self) -> SingleValueMapper<BigUint>;

    // Staking Provider
    #[view(getDelegateAddress)]
    #[storage_mapper("delegate_address")]
    fn delegate_address(&self) -> SingleValueMapper<ManagedAddress>;
}