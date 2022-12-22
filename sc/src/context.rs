elrond_wasm::imports!();
elrond_wasm::derive_imports!();


#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct UnstakingPack<M: ManagedTypeApi>  {
    pub amount: BigUint<M>,
    pub timestamp: u64,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct LiquidStakingSettings<M: ManagedTypeApi>  {
    pub vegld_identifier: TokenIdentifier<M>,
    pub treasury_wallet: ManagedAddress<M>,
    pub fee: u64,
    pub unbonding_period: u64,
    pub admins: ManagedVec<M, ManagedAddress<M>>,
    pub user_action_allowed: bool,
    pub admin_action_allowed: bool,
    pub is_token_roles_set: bool,
    pub auto_delegate_address: ManagedAddress<M>,

    pub pool_vegld_amount: BigUint<M>,
    pub pool_egld_amount: BigUint<M>,
    pub prestaked_egld_amount: BigUint<M>,
    pub preunstaked_egld_amount: BigUint<M>,
    pub unstaking_egld_amount: BigUint<M>,
    pub unbonded_egld_amount: BigUint<M>,

    pub vegld_price: BigUint<M>,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, ManagedVecItem, Clone)]
pub struct UserUnstakingPacks<M: ManagedTypeApi>  {
    pub address: ManagedAddress<M>,
    pub packs: ManagedVec<M, UnstakingPack<M>>,
}