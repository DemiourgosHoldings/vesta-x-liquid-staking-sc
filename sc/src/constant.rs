pub const TOKEN_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
pub const ONE_EGLD_IN_WEI: u64 = 1_000_000_000_000_000_000;
pub const MAX_PERCENTAGE: u64 = 100_00;

// Vegld Token
pub const VEGLD_DISPLAY_NAME: &[u8] = b"VestaXStakedEGLD";
pub const VEGLD_TICKER: &[u8] = b"VEGLD";
pub const VEGLD_DECIMALS: usize = 18;
pub const ONE_VEGLD_IN_WEI: u64 = 1_000_000_000_000_000_000;

// Delegate
pub const DELEGATE_MIN_AMOUNT: u64 = 1_000_000_000_000_000_000;

// Unbond
pub const MIN_UNBONDING_PERIOD: u64 = 3600 * 24 * 10;   // 10 days
pub const MAX_UNBONDING_PERIOD: u64 = 3600 * 24 * 30;  // 30 days

// user withdraw
pub const MAX_LOOP_IN_USER_WITHDRAW: usize = 10;

// async call check
pub const MAX_BLOCKS_FOR_ASYNC_CALLBACK: u64 = 10;  // 10 blocks = 60 seconds

// gas limit
pub const MIN_GAS_FOR_ASYNC_CALL: u64 = 20_000_000;
pub const MIN_GAS_FOR_CALLBACK: u64 = 20_000_000;

//
pub const METACHAIN_SHARD_ID: u32 = core::u32::MAX;

//
pub const EXPIRATION_EPOCH_COUNT: u64 = 5;