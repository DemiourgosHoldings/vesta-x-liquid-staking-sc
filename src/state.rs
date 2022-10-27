elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct UnstakingPack<M: ManagedTypeApi>  {
    pub amount: BigUint<M>,
    pub timestamp: u64,
}