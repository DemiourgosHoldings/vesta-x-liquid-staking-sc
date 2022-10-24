elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{ UserActionItem };

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

    // prestaked egld amount of each wallet
    #[view(getPrestakedEgldAmountMap)]
    #[storage_mapper("prestaked_egld_amount_map")]
    fn prestaked_egld_amount_map(&self) -> MapMapper<ManagedAddress, BigUint>;


    /* PreUnstake Pool */
    // total preunstaked VALAR amount
    #[view(getPreunstakedValarAmount)]
    #[storage_mapper("preunstaked_valar_amount")]
    fn preunstaked_valar_amount(&self) -> SingleValueMapper<BigUint>;

    // preunstaked VALAR amount queue
    #[view(getPreunstakedValarAmountQueue)]
    #[storage_mapper("preunstaked_valar_amount_queue")]
    fn preunstaked_valar_amount_queue(&self) -> LinkedListMapper<UserActionItem<Self::Api>>;


    /* Unbonding Pool */
    // total unbonding EGLD amount
    #[view(getUnbondingEgldAmount)]
    #[storage_mapper("unbonding_egld_amount")]
    fn unbonding_egld_amount(&self) -> SingleValueMapper<BigUint>;

    // unbonding EGLD amount queue
    #[view(getUnbondingEgldAmountQueue)]
    #[storage_mapper("unbonding_egld_amount_queue")]
    fn unbonding_egld_amount_queue(&self) -> LinkedListMapper<UserActionItem<Self::Api>>;


    /* Unbonded Pool */
    // total unbonded EGLD amount
    #[view(getUnbondedEgldAmount)]
    #[storage_mapper("unbonded_egld_amount")]
    fn unbonded_egld_amount(&self) -> SingleValueMapper<BigUint>;

    // unbonded EGLD amount map
    #[view(getUnbondedEgldAmountMap)]
    #[storage_mapper("unbonded_egld_amount_map")]
    fn unbonded_egld_amount_map(&self) -> MapMapper<ManagedAddress, BigUint>;
}