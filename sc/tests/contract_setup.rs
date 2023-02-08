use elrond_wasm::{
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole},
};
use elrond_wasm_debug::{
    managed_address, managed_token_id, rust_biguint,
    testing_framework::*,
    // tx_mock::TxResult,
    DebugApi,
};
use vesta_x_liquid_staking::{
    VestaXLiquidStaking,
    config::ConfigModule,
    storages::common_storage::CommonStorageModule
};

pub const LIQUID_STAKING_WASM_PATH: &str = "sc/output/vesta-x-liquid-staking.wasm";

pub const UNBONDING_PERIOD: u64 = 3600 * 24 * 11;   // 11 days
pub const FEE: u64 = 500;     // 5%
pub const MAX_PERCENTAGE: u64 = 100_00;
pub const START_TIMESTAMP: u64 = 0;

pub static VEGLD_TOKEN_ID: &[u8] = b"VEGLD-123456";

pub static ESDT_ROLES: &[EsdtLocalRole] = &[
    EsdtLocalRole::Mint,
    EsdtLocalRole::Burn,
    EsdtLocalRole::Transfer,
];

pub struct VestaXLiquidStakingSetup<VestaXLiquidStakingContractObjBuilder>
where
VestaXLiquidStakingContractObjBuilder: 'static + Copy + Fn() -> vesta_x_liquid_staking::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub treasury_wallet: Address,
    pub sc_wrapper:
        ContractObjWrapper<vesta_x_liquid_staking::ContractObj<DebugApi>, VestaXLiquidStakingContractObjBuilder>,
}

impl<VestaXLiquidStakingContractObjBuilder> VestaXLiquidStakingSetup<VestaXLiquidStakingContractObjBuilder>
where
    VestaXLiquidStakingContractObjBuilder: 'static + Copy + Fn() -> vesta_x_liquid_staking::ContractObj<DebugApi>,
{
    pub fn new(builder: VestaXLiquidStakingContractObjBuilder) -> Self {
        let rust_zero = rust_biguint!(0);
        let mut b_mock = BlockchainStateWrapper::new();
        let owner_address = b_mock.create_user_account(&rust_zero);
        let treasury_wallet = b_mock.create_user_account(&rust_zero);

        let sc_wrapper =
            b_mock.create_sc_account(
                &rust_zero,
                Some(&owner_address),
                builder,
                LIQUID_STAKING_WASM_PATH
            );

        b_mock.set_block_timestamp(START_TIMESTAMP);

        // init
        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.init();
            })
            .assert_ok();

        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.set_settings(
                    UNBONDING_PERIOD,
                    managed_address!(&treasury_wallet),
                    FEE,
                    true,
                    true,
                );
            })
            .assert_ok();
        
        b_mock
            .execute_tx(&owner_address, &sc_wrapper, &rust_zero, |sc| {
                sc.vegld_identifier().set_token_id(managed_token_id!(VEGLD_TOKEN_ID));
            })
            .assert_ok();
        
        b_mock.set_esdt_local_roles(sc_wrapper.address_ref(), VEGLD_TOKEN_ID, ESDT_ROLES);

        Self {
            b_mock,
            owner_address,
            treasury_wallet,
            sc_wrapper,
        }
    }
}
