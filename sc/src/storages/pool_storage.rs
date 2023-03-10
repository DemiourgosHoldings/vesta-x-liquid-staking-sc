multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::context::{ UnstakingPack };

#[multiversx_sc::module]
pub trait PoolStorageModule
{
    /* LP Share Pool - Main Pool */
    // VEGLD token total supply
    #[view(getPoolVegldAmount)]
    #[storage_mapper("pool_vegld_amount")]
    fn pool_vegld_amount(&self) -> SingleValueMapper<BigUint>;

    // total delegated EGLD amount in Delegate SCs (excluding undelegating or undelegated EGLD)
    #[view(getPoolEgldAmount)]
    #[storage_mapper("pool_egld_amount")]
    fn pool_egld_amount(&self) -> SingleValueMapper<BigUint>;

    /* PreStake Pool */
    // total prestaked amount
    #[view(getPrestakedEgldAmount)]
    #[storage_mapper("prestaked_egld_amount")]
    fn prestaked_egld_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getPendingRewardEgldAmount)]
    #[storage_mapper("pending_reward_egld_amount")]
    fn pending_reward_egld_amount(&self) -> SingleValueMapper<BigUint>;

    /* PreUnstake Pool */
    // total preunstaked EGLD amount
    #[view(getPreunstakedEgldAmount)]
    #[storage_mapper("preunstaked_egld_amount")]
    fn preunstaked_egld_amount(&self) -> SingleValueMapper<BigUint>;

    /* Unstaking Packs */
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


    //////////////////////////////////////////////////////////////
    #[view(getTotalUndelegatedEgldAmount)]
    #[storage_mapper("total_undelegated_egld_amount")]
    fn total_undelegated_egld_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getTotalOldPreunstakedEgldAmount)]
    #[storage_mapper("total_old_preunstaked_egld_amount")]
    fn total_old_preunstaked_egld_amount(&self) -> SingleValueMapper<BigUint>;

    #[view(getRecentPreunstakedEgldAmountsMap)]
    #[storage_mapper("recent_preunstaked_egld_amounts_map")]
    fn recent_preunstaked_egld_amounts_map(&self) -> MapMapper<u64, BigUint>;
}