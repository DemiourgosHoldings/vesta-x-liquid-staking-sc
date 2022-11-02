elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{ UnstakingPack };

#[elrond_wasm::module]
pub trait PoolStorageModule
{
    /* LP Share Pool - Main Pool */
    // VALAR token total supply
    #[view(getPoolValarAmount)]
    #[storage_mapper("pool_valar_amount")]
    fn pool_valar_amount(&self) -> SingleValueMapper<BigUint>;

    // total delegated EGLD amount in Delegate SCs (excluding undelegating or undelegated EGLD)
    #[view(getPoolEgldAmount)]
    #[storage_mapper("pool_egld_amount")]
    fn pool_egld_amount(&self) -> SingleValueMapper<BigUint>;


    /* PreStake Pool */
    // total prestaked amount
    #[view(getPrestakedEgldAmount)]
    #[storage_mapper("prestaked_egld_amount")]
    fn prestaked_egld_amount(&self) -> SingleValueMapper<BigUint>;


    /* PreUnstake Pool */
    // total preunstaked EGLD amount
    #[view(getPreunstakedEgldAmount)]
    #[storage_mapper("preunstaked_egld_amount")]
    fn preunstaked_egld_amount(&self) -> SingleValueMapper<BigUint>;


    /* Unstaking Packs */
    // total unstaking EGLD amount
    #[view(getUnstakingEgldAmount)]
    #[storage_mapper("unstaking_egld_amount")]
    fn unstaking_egld_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getUnstakingUsers)]
    #[storage_mapper("unstaking_users")]
    fn unstaking_users(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getUnstakingPacks)]
    #[storage_mapper("unstaking_packs")]
    fn unstaking_packs(&self, user: &ManagedAddress) -> LinkedListMapper<UnstakingPack<Self::Api>>;

    /* Unbonded Pool */
    // total unbonded EGLD amount
    #[view(getUnbondedEgldAmount)]
    #[storage_mapper("unbonded_egld_amount")]
    fn unbonded_egld_amount(&self) -> SingleValueMapper<BigUint>;
}