elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait PoolModule:
    crate::storage::StorageModule
    + crate::event::EventModule
{
    #[inline]
    fn mint_and_send_valar(&self, to: &ManagedAddress, egld_amount: &BigUint) -> BigUint {
        let valar_supply = self.valar_supply().get();
        let staked_egld_amount = self.staked_egld_amount().get();

        let valar_mint_amount = if valar_supply == BigUint::zero() { // First Mint
            egld_amount.clone()
        } else {
            require!(
                staked_egld_amount != BigUint::zero(),
                "staked_egld_amount is zero while valar_supply is not zero."
            );

            self.quote_valar(egld_amount)
        };

        //
        self.valar_identifier().mint_and_send(to, valar_mint_amount.clone());

        //
        self.valar_supply().set(valar_supply + &valar_mint_amount);
        self.staked_egld_amount().set(staked_egld_amount + egld_amount);

        valar_mint_amount
    }

    #[inline]
    fn burn_valar(&self, valar_amount: &BigUint, egld_amout: &BigUint) {
        self.send().esdt_local_burn(&self.valar_identifier().get_token_id(), 0, valar_amount);
        self.valar_supply().update(|v| *v -= valar_amount);
        self.staked_egld_amount().update(|v| *v -= egld_amout);
    }

    #[view(quoteValar)]
    fn quote_valar(&self, egld_amount: &BigUint) -> BigUint {
        self.valar_supply().get() * egld_amount / &self.staked_egld_amount().get()
    }

    #[view(quoteEgld)]
    fn quote_egld(&self, valar_amount: &BigUint) -> BigUint {
        self.staked_egld_amount().get() * valar_amount / &self.valar_supply().get()
    }
}