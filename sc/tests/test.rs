use elrond_wasm_debug::{
    DebugApi,
};

pub mod contract_setup;
pub mod contract_interactions;

use contract_setup::*;

#[test]
fn init_test() {
    let _ = VestaXLiquidStakingSetup::new(vesta_x_liquid_staking::contract_obj);
}

#[test]
fn user_stake_and_unstake_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = VestaXLiquidStakingSetup::new(vesta_x_liquid_staking::contract_obj);

    let stake_amount = 1000u64;
    let first_user = sc_setup.setup_new_user(stake_amount);

    sc_setup.user_stake(&first_user, stake_amount);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
    sc_setup.check_contract_storage(
        stake_amount,
        stake_amount,
        stake_amount,
        0u64,
        0u64,
    );

    let unstake_amount = 200u64;
    sc_setup.user_unstake(&first_user, VEGLD_TOKEN_ID, unstake_amount);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
    sc_setup.check_contract_storage(
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        stake_amount,
        unstake_amount,
        0u64,
    );
}

#[test]
fn user_stake_and_unstake_and_withdraw_from_prestaked_and_withdraw_test() {
    let _ = DebugApi::dummy();
    let mut sc_setup = VestaXLiquidStakingSetup::new(vesta_x_liquid_staking::contract_obj);

    let stake_amount = 1000u64;
    let first_user = sc_setup.setup_new_user(stake_amount);

    // stake
    sc_setup.user_stake(&first_user, stake_amount);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
    sc_setup.check_contract_storage(
        stake_amount,
        stake_amount,
        stake_amount,
        0u64,
        0u64,
    );

    // unstake
    let unstake_amount = 200u64;
    sc_setup.user_unstake(&first_user, VEGLD_TOKEN_ID, unstake_amount);
    sc_setup.check_user_egld_balance(&first_user, 0u64);
    sc_setup.check_contract_storage(
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        stake_amount,
        unstake_amount,
        0u64,
    );

    // withdraw_from_prestaked
    sc_setup.withdraw_from_prestaked(&sc_setup.owner_address.clone());
    sc_setup.check_user_egld_balance(&first_user, 0u64);
    sc_setup.check_contract_storage(
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        0u64,
        unstake_amount,
    );

    // early withdraw will cause panic
    sc_setup.user_withdraw_too_early(&first_user);

    let current_timestamp = START_TIMESTAMP + UNBONDING_PERIOD + 1u64;
    sc_setup.set_block_timestamp(current_timestamp);

    // withdraw after UNBONDING_PERIOD
    sc_setup.user_withdraw(&first_user);
    sc_setup.check_user_egld_balance(&first_user, unstake_amount);
    sc_setup.check_contract_storage(
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        stake_amount - unstake_amount,
        0u64,
        0u64,
    );
}