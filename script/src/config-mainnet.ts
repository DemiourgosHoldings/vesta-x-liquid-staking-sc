// export const GATEWAY_URL = "https://devnet-gateway.elrond.com";
// export const API_URL = "https://devnet-api.elrond.com";
// export const EXPLORER_URL = 'https://devnet-explorer.elrond.com/transactions/';
// export const CHAIN_ID = "D"

// export const GATEWAY_URL = "https://testnet-gateway.elrond.com";
// export const API_URL = "https://testnet-api.elrond.com";
// export const EXPLORER_URL = 'https://testnet-explorer.elrond.com/transactions/';
// export const CHAIN_ID = "T"

export const GATEWAY_URL = "https://gateway.elrond.com";
export const API_URL = "https://api.elrond.com";
export const EXPLORER_URL = 'https://explorer.elrond.com/transactions/';
export const CHAIN_ID = "1"

export const PEM_PATH = "./wallets/vestax-vegld.pem";

// SC metadata
export const SMART_CONRACT_ABI_URL = 'abi/vesta-x-liquid-staking.abi.json';
export const SMART_CONRACT_NAME = 'VestaXLiquidStaking';
export const SMART_CONRACT_ADDRESS = 'erd1qqqqqqqqqqqqqpgqawus4zu5w2frmhh9rscjqnu9x6msfjya2d2sfw7tsn';

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
export const DELEGATE_TO_SP_GAS_LIMIT = 30_000_000;
export const ADMIN_UNDELEGATE_GAS_LIMIT = 30_000_000;
export const ADMIN_WITHDRAW_GAS_LIMIT = 30_000_000;
export const ADMIN_CLAIM_REWARDS_GAS_LIMIT = 30_000_000;

//
export const UNBONDING_PERIOD = 3600 * 2; // 2 hours
export const TREASURY_WALLET = 'erd1qqqqqqqqqqqqqpgqc0e2y2fsv8mcxja6vqz7k7wh33mtclj027rsxry8a2';    // DAO Vault
export const FEE = 500; // 5%
export const ADMIN_ADDRESSES = [
    'erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay', // my shard1
    'erd1klkykugf5w67nrj22fvkalzf34rge7czr4cd93rj6rdz9t8upd4sww53ju', // vestax-admin
];
export const USER_ACTION_ALLOWED = true;
export const MANAGEMENT_ACTION_ALLOWED = true;

//
export const VEGLD_TOKEN = 'VEGLD-2b9319';
export const VEGLD_DECIMALS = 18;
export const VEGLD_TICKER = 'vEGLD';

//
export const DELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq50lllls8dl5vl'; // should be Parasco in default
export const UNDELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq50lllls8dl5vl'; // should be Parasco in default
export const WHITELISTED_SP_ADDRESSES = [
    'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqq50lllls8dl5vl', // Parasco SP
];

//
export const USER_STAKE_AMOUNT = 1_000_000_000_000_000_000;
export const USER_UNSTAKE_AMOUNT = 1_000_000_000_000_000_000;

//
export const FAILED_ASYNC_CALL_IDS = [1, 2];