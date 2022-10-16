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
    
}