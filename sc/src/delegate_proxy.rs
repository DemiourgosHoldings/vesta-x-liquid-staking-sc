#![allow(non_snake_case)]

multiversx_sc::imports!();

#[multiversx_sc::proxy]
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