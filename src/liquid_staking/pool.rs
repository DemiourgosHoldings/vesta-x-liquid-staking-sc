elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[elrond_wasm::module]
pub trait PoolModule:
crate::storages::common_storage::CommonStorageModule
+ crate::storages::pool_storage::PoolStorageModule
+ crate::event::EventModule
+ crate::amm::AmmModule
{    
    #[only_owner]
    #[endpoint(updateMainPool)]
    fn update_main_pool(&self, pool_valar_amount: BigUint, pool_egld_amount: BigUint,) {
        self.pool_valar_amount().set(&pool_valar_amount);
        self.pool_egld_amount().set(&pool_egld_amount);

        self.update_main_pool_event(&self.blockchain().get_caller(), &pool_valar_amount, &pool_egld_amount);
    }   
}