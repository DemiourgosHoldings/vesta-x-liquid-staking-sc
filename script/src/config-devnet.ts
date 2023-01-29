export const GATEWAY_URL = "https://devnet-gateway.elrond.com";
export const API_URL = "https://devnet-api.elrond.com";
export const EXPLORER_URL = 'https://devnet-explorer.elrond.com/transactions/';
export const CHAIN_ID = "D"

// export const GATEWAY_URL = "https://testnet-gateway.elrond.com";
// export const API_URL = "https://testnet-api.elrond.com";
// export const EXPLORER_URL = 'https://testnet-explorer.elrond.com/transactions/';
// export const CHAIN_ID = "T"

// export const GATEWAY_URL = "https://gateway.elrond.com";
// export const API_URL = "https://api.elrond.com";
// export const EXPLORER_URL = 'https://explorer.elrond.com/transactions/';
// export const CHAIN_ID = "1"

export const PEM_PATH = "./wallets/shard1-test2.pem";

// SC metadata
export const SMART_CONRACT_ABI_URL = 'abi/vesta-x-liquid-staking.abi.json';
export const SMART_CONRACT_NAME = 'VestaXLiquidStaking';
export const SMART_CONRACT_ADDRESS = 'erd1qqqqqqqqqqqqqpgqsvunnqer0q8uecrf4wqfk6vm2x78d49803as452f4r';

//
export const ESDT_MODULE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u';
export const TOKEN_ISSUE_COST = 0.05; // 0.05 EGLD
export const ZERO_ADDRESS = 'erd1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq6gq4hu';

//
export const SET_SETTINGS_GAS_LIMIT = 6_000_000;
export const ISSUE_GAS_LIMIT = 100_000_000;

// export const USER_STAKE_GAS_LIMIT = 6_000_000;   // when auto-delegate disalbled
export const USER_STAKE_GAS_LIMIT = 25_000_000;     // when auto-delegate enabled

export const USER_UNSTAKE_GAS_LIMIT = 25_000_000;
export const USER_WITHDRAW_GAS_LIMIT = 15_000_000;
export const ADMIN_DELEGATE_GAS_LIMIT = 30_000_000;
export const ADMIN_UNDELEGATE_GAS_LIMIT = 30_000_000;
export const ADMIN_WITHDRAW_GAS_LIMIT = 30_000_000;
export const ADMIN_CLAIM_REWARDS_GAS_LIMIT = 30_000_000;

//
export const UNBONDING_PERIOD = 3600 * 2; // 2 hours
export const TREASURY_WALLET = 'erd15936k9pw34xyzmcaupyn7lpr7f6p20q50h4wlgemxg7h9zasdfysmhg50z';    // shard1-odin
export const FEE = 500; // 5%
export const ADMIN_ADDRESSES = [
    'erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay', // my shard1
    'erd1klkykugf5w67nrj22fvkalzf34rge7czr4cd93rj6rdz9t8upd4sww53ju', // vestax-admin
];
export const USER_ACTION_ALLOWED = true;
export const ADMIN_ACTION_ALLOWED = true;

//
export const VEGLD_TOKEN = 'VEGLD-a99e98';
export const VEGLD_DECIMALS = 18;
export const VEGLD_TICKER = 'vEGLD';

//
export const DELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py';
export const UNDELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py';
export const WHITELISTED_SP_ADDRESSES = [
    'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py',
];

//
export const USER_STAKE_AMOUNT = 10_000_000_000_000_000_000;
export const USER_UNSTAKE_AMOUNT = 1_000_000_000_000_000_000;

//
export const FAILED_ASYNC_CALL_IDS = [1, 2];