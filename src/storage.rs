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

    #[view(getUnbondingEgldAmount)]
    #[storage_mapper("unbonding_egld_amount")]
    fn unbonding_egld_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnbondedEgldAmount)]
    #[storage_mapper("unbonded_egld_amount")]
    fn unbonded_egld_amount(&self) -> SingleValueMapper<BigUint>;

    // User
    #[view(getUnbondingUsers)]
    #[storage_mapper("unbonding_users")]
    fn unbonding_users(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUnbondingEgldAmountPerUser)]
    #[storage_mapper("unbonding_egld_amount_per_user")]
    fn unbonding_egld_amount_per_user(&self, user: &ManagedAddress) -> MapMapper<u64, BigUint>; // unbonding started timestamp and unbonding amount

    #[view(getUnbondedEgldAmountPerUser)]
    #[storage_mapper("unbonded_egld_amount_per_user")]
    fn unbonded_egld_amount_per_user(&self) -> MapMapper<ManagedAddress, BigUint>;

    // Staking Provider
    #[view(getDelegateAddress)]
    #[storage_mapper("delegate_address")]
    fn delegate_address(&self) -> SingleValueMapper<ManagedAddress>;
}