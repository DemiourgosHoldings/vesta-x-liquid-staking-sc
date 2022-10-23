elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait CommonStorageModule
{
    // Valar Token
    #[view(getValarIdentifier)]
    #[storage_mapper("valar_identifier")]
    fn valar_identifier(&self) -> FungibleTokenMapper<Self::Api>;

    // Staking Provider
    #[view(getDelegateAddress)]
    #[storage_mapper("delegate_address")]
    fn delegate_address(&self) -> SingleValueMapper<ManagedAddress>;

    //
    #[view(getTreasuryWallet)]
    #[storage_mapper("treasury_wallet")]
    fn treasury_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    //
    #[view(getAutoDelegateEnabled)]
    #[storage_mapper("auto_delegate_enabled")]
    fn auto_delegate_enabled(&self) -> SingleValueMapper<bool>;
}