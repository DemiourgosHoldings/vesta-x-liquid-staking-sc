elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct UserActionItem<M: ManagedTypeApi>  {
    address: ManagedAddress<M>,
    amount: BigUint<M>,
    timestamp: u64,
}