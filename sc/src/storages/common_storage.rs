elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait CommonStorageModule
{
    // Liquid Staking Token
    #[view(getVegldIdentifier)]
    #[storage_mapper("vegld_identifier")]
    fn vegld_identifier(&self) -> FungibleTokenMapper<Self::Api>;

    //
    #[view(getTreasuryWallet)]
    #[storage_mapper("treasury_wallet")]
    fn treasury_wallet(&self) -> SingleValueMapper<ManagedAddress>;

    //
    #[view(getFee)]
    #[storage_mapper("fee")]
    fn fee(&self) -> SingleValueMapper<u64>;

    //
    #[view(getUnbondingPeriod)]
    #[storage_mapper("unbonding_period")]
    fn unbonding_period(&self) -> SingleValueMapper<u64>;

    //
    #[view(getAdmins)]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    //
    #[view(getUserActionAllowed)]
    #[storage_mapper("user_action_allowed")]
    fn user_action_allowed(&self) -> SingleValueMapper<bool>;

    #[view(getAdminActionAllowed)]
    #[storage_mapper("admin_action_allowed")]
    fn admin_action_allowed(&self) -> SingleValueMapper<bool>;

    ///////////////////////////////////////////////////////////

    // if this address is given, user stakes will be auto-delegated
    #[view(getAutoDelegateAddress)]
    #[storage_mapper("auto_delegate_address")]
    fn auto_delegate_address(&self) -> SingleValueMapper<ManagedAddress>;

    // if this address is given, user stakes will be auto-delegated
    #[view(getAutoUndelegateAddress)]
    #[storage_mapper("auto_undelegate_address")]
    fn auto_undelegate_address(&self) -> SingleValueMapper<ManagedAddress>;
}