use multiversx_sc::{
    storage::mappers::StorageTokenWrapper,
    types::{Address, EsdtLocalRole, BigUint},
};
use multiversx_sc_scenario::{
    managed_address, managed_token_id, rust_biguint, num_bigint,
    testing_framework::*,
    // tx_mock::TxResult,
    DebugApi,
};
use vesta_x_liquid_staking::{
    VestaXLiquidStaking,
    config::ConfigModule,
    storages::common_storage::CommonStorageModule,
    storages::pool_storage::PoolStorageModule,
    user::UserModule,
    management::ManagementModule,
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
