elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::state::{ UserActionItem };

#[elrond_wasm::module]
pub trait ViewUserModule:
    crate::storages::common_storage::CommonStorageModule
    + crate::storages::pool_storage::PoolStorageModule
    + crate::amm::AmmModule
{
    ///
    #[view(viewPrestakedAmount)]
    fn view_prestaked_amount(&self, caller: ManagedAddress) -> BigUint {
        match self.prestaked_egld_amount_map().get(&caller) {
            Some(v) => v,
            None => BigUint::zero(),
        }
    }

    ///
    #[view(viewPreunstakedPacks)]
    fn view_preunstaked_packs(&self, caller: ManagedAddress) -> ManagedVec<Self::Api, UserActionItem<Self::Api>> {
        let preunstaked_valar_amount_queue = self.preunstaked_valar_amount_queue();
        let mut packs = ManagedVec::new();
        for node in preunstaked_valar_amount_queue.iter() {
            let item = node.get_value_as_ref();
            if caller == item.address {
                packs.push(item.clone())
            }
        }
        
        packs
    }
    
    ///
    #[view(viewUnbondingPacks)]
    fn view_unbonding_packs(&self, caller: ManagedAddress) -> ManagedVec<Self::Api, UserActionItem<Self::Api>> {
        let unbonding_egld_amount_queue = self.unbonding_egld_amount_queue();
        let mut packs = ManagedVec::new();
        for node in unbonding_egld_amount_queue.iter() {
            let item = node.get_value_as_ref();
            if caller == item.address {
                packs.push(item.clone())
            }
        }
        
        packs
    }

    ///
    #[view(viewUnbondedAmount)]
    fn view_unbonded_amount(&self, caller: ManagedAddress) -> BigUint {
        match self.unbonded_egld_amount_map().get(&caller) {
            Some(v) => v,
            None => BigUint::zero(),
        }
    }
}