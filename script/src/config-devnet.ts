export const GATEWAY_URL = "https://devnet-gateway.multiversx.com";
export const API_URL = "https://devnet-api.multiversx.com";
export const EXPLORER_URL = 'https://devnet-explorer.multiversx.com/transactions/';
export const CHAIN_ID = "D"

// export const GATEWAY_URL = "https://testnet-gateway.elrond.com";
// export const API_URL = "https://testnet-api.elrond.com";
// export const EXPLORER_URL = 'https://testnet-explorer.elrond.com/transactions/';
// export const CHAIN_ID = "T"

// export const GATEWAY_URL = "https://gateway.elrond.com";
// export const API_URL = "https://api.elrond.com";
// export const EXPLORER_URL = 'https://explorer.elrond.com/transactions/';
// export const CHAIN_ID = "1"

export const PEM_PATH = "../../wallet-DH/owner/erd14kgcxv7kev8evnnly4ctnweaf7f45jmff7n52ajmgpmxu7u9c5wsp6f0jk.pem";

// SC metadata
export const SMART_CONRACT_ABI_URL = 'abi/vesta-x-liquid-staking.abi.json';
export const SMART_CONRACT_NAME = 'VestaXLiquidStaking';
export const SMART_CONRACT_ADDRESS = 'erd1qqqqqqqqqqqqqpgq7mvz9c6uwwv4y7eee8gcz4588c2c595zc5wspz0tek';

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
export const COMMON_CONFIG_GAS_LIMIT = 20_000_000;

//
export const UNBONDING_PERIOD = 864000 * 2; // 2 hours
export const TREASURY_WALLET = 'erd1lp8qlwx5s480yefu32wvw9ehkpdvznslq8unj7usfugalp50suhst0ytdn';    // shard1-odin
export const FEE = 500; // 5%
export const ADMIN_ADDRESSES = [
    'erd14kgcxv7kev8evnnly4ctnweaf7f45jmff7n52ajmgpmxu7u9c5wsp6f0jk'
];
export const USER_ACTION_ALLOWED = true;
export const MANAGEMENT_ACTION_ALLOWED = true;

//
export const VEGLD_TOKEN = 'VEGLD-103b9f';
export const VEGLD_DECIMALS = 18;
export const VEGLD_TICKER = 'vEGLD';

//
export const DELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py';
export const UNDELEGATE_ADDRESS = 'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py';
export const WHITELISTED_SP_ADDRESSES = [
    'erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqr0llllsj732py',
];

//
export const USER_STAKE_AMOUNT = 1_000_000_000_000_000_000;
export const USER_UNSTAKE_AMOUNT = 1_000_000_000_000_000_000;

//
export const FAILED_ASYNC_CALL_IDS = [1, 2];