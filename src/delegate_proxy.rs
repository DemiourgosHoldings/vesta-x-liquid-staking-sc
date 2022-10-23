#![allow(non_snake_case)]

elrond_wasm::imports!();

#[elrond_wasm::proxy]
pub trait DelegateProxy {

    #[payable("EGLD")]
    #[endpoint(delegate)]
    fn delegate(&self);

    #[endpoint(unDelegate)]
    fn unDelegate(
        &self,
        amount: BigUint
    );

    #[endpoint(reDelegateRewards)]
    fn reDelegateRewards(
        &self
    );

    #[endpoint(withdraw)]
    fn withdraw(
        &self
    );

    #[endpoint(claimRewards)]
    fn claimRewards(
        &self
    );
}