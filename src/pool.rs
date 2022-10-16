elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait PoolModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[inline]
    fn mint_valar(&self, egld_amount: &BigUint) -> BigUint{
        let valar_supply = self.valar_supply().get();
        let staked_egld_amount = self.staked_egld_amount().get();

        let valar_mint_amount = if valar_supply == BigUint::zero() { // First Mint
            egld_amount.clone()
        } else {
            require!(
                staked_egld_amount != BigUint::zero(),
                "staked_egld_amount is zero while valar_supply is not zero."
            );

            valar_supply.clone() * egld_amount / &staked_egld_amount
        };

        //
        self.send().esdt_local_mint(
            &self.valar_identifier().get_token_id(),
            0,
            &valar_mint_amount,
        );

        //
        self.valar_supply().set(valar_supply + &valar_mint_amount);
        self.staked_egld_amount().set(staked_egld_amount + egld_amount);

        valar_mint_amount
    }
}