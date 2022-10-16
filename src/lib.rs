#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait ValarLiquidStaking {
    #[init]
    fn init(&self) {
    }
}
