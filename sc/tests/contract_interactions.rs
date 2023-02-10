use elrond_wasm::{
    types::{Address, BigUint},
};
use elrond_wasm_debug::{
    rust_biguint, num_bigint,
    DebugApi,
};
use vesta_x_liquid_staking::{
    liquid_staking::user::UserModule,
    liquid_staking::management::ManagementModule,
    storages::pool_storage::PoolStorageModule,
};
use crate::contract_setup::*;

impl<VestaXLiquidStakingContractObjBuilder> VestaXLiquidStakingSetup<VestaXLiquidStakingContractObjBuilder>
where
VestaXLiquidStakingContractObjBuilder: 'static + Copy + Fn() -> vesta_x_liquid_staking::ContractObj<DebugApi>,
{
    pub fn setup_new_user(&mut self, egld_mount: u64) -> Address {
        let rust_zero = rust_biguint!(0);

        let new_user = self.b_mock.create_user_account(&rust_zero);
        self.b_mock
            .set_egld_balance(&new_user, &num_bigint::BigUint::from(egld_mount));
        new_user
    }

    pub fn set_block_timestamp(&mut self, current_timestamp: u64) {
        self.b_mock
            .set_block_timestamp(current_timestamp);
    }

    pub fn check_user_egld_balance(&self, address: &Address, token_balance: u64) {
        self.b_mock
            .check_egld_balance(address, &num_bigint::BigUint::from(token_balance));
    }

    pub fn check_contract_storage(
        &mut self,
        pool_vegld_amount: u64,
        pool_egld_amount: u64,
        prestaked_egld_amount: u64,
        preunstaked_egld_amount: u64,
        unbonded_egld_amount: u64,
    ) {
        self.b_mock
            .execute_query(&self.sc_wrapper, |sc| {
                assert_eq!(
                    sc.pool_vegld_amount().get(),
                    BigUint::from(pool_vegld_amount)
                );
                assert_eq!(
                    sc.pool_egld_amount().get(),
                    BigUint::from(pool_egld_amount)
                );
                assert_eq!(
                    sc.prestaked_egld_amount().get(),
                    BigUint::from(prestaked_egld_amount)
                );
                assert_eq!(
                    sc.preunstaked_egld_amount().get(),
                    BigUint::from(preunstaked_egld_amount)
                );
                assert_eq!(
                    sc.unbonded_egld_amount().get(),
                    BigUint::from(unbonded_egld_amount)
                );
            })
            .assert_ok();
    }

    pub fn user_stake(
        &mut self,
        caller: &Address,
        payment_amount: u64
    ) {
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &num_bigint::BigUint::from(payment_amount),
                |sc| {
                    sc.user_stake();
                },
            )
            .assert_ok();
    }

    pub fn user_unstake(
        &mut self,
        caller: &Address,
        payment_token: &[u8],
        payment_amount: u64,
    ) {
        self.b_mock
            .execute_esdt_transfer(
                caller,
                &self.sc_wrapper,
                payment_token,
                0,
                &num_bigint::BigUint::from(payment_amount),
                |sc| {
                    sc.user_unstake();
                },
            )
            .assert_ok();
    }

    pub fn user_withdraw(
        &mut self,
        caller: &Address,
    ) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &rust_zero,
                |sc| {
                    sc.user_withdraw();
                },
            )
            .assert_ok();
    }

    pub fn user_withdraw_too_early(
        &mut self,
        caller: &Address
    ) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &rust_zero,
                |sc| {
                    sc.user_withdraw();
                },
            )
            .assert_error(4, "No EGLD to withdraw.");
    }


    pub fn withdraw_from_prestaked(
        &mut self,
        caller: &Address,
    ) {
        let rust_zero = rust_biguint!(0u64);
        self.b_mock
            .execute_tx(
                caller,
                &self.sc_wrapper,
                &rust_zero,
                |sc| {
                    sc.withdraw_from_prestaked();
                },
            )
            .assert_ok();
    }
}
