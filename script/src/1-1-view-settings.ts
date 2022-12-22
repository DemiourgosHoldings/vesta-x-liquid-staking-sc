import * as fs from 'fs';
import { sendTransactions, timedOutBatchTransactionsStates } from "@elrondnetwork/dapp-core";
import {
	Account,
	Address,
	AddressValue,
	ChainID,
	ContractFunction,
	GasLimit,
	I8Value,
	ProxyProvider,
	SmartContract,
	StringValue,
	AbiRegistry,
	SmartContractAbi,
	Egld,
	Balance,
	BigUIntValue,
	BytesValue,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	U64Value,
	U32Value,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
} from "./config";

import {
	account,
	provider,
	signer,
	getSmartContractInteractor,
} from './provider';
import BigNumber from 'bignumber.js';
import {
	sleep,
	convertWeiToEsdt,
	convertEsdtToWei,
} from './util';

async function main() {
	const contractInteractor = await getSmartContractInteractor();
	const interaction = contractInteractor.contract.methods.viewLiquidStakingSettings();
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
        console.log('res', res);
        return;
    }
	const value = res.firstValue.valueOf();
	
    const decoded = {
        vegld_identifier: value.vegld_identifier.toString(),
        treasury_wallet: value.treasury_wallet.toString(),
        fee: value.fee.toNumber(),
        unbonding_period: value.unbonding_period.toNumber(),
        admins: value.admins.map(v => v.toString()),
        user_action_allowed: value.user_action_allowed,
        admin_action_allowed: value.admin_action_allowed,
        is_token_roles_set: value.is_token_roles_set,

        pool_vegld_amount: value.pool_vegld_amount.toFixed(),
        pool_egld_amount: value.pool_egld_amount.toFixed(),
        prestaked_egld_amount: value.prestaked_egld_amount.toFixed(),
        preunstaked_egld_amount: value.preunstaked_egld_amount.toFixed(),
        unstaking_egld_amount: value.unstaking_egld_amount.toFixed(),
        unbonded_egld_amount: value.unbonded_egld_amount.toFixed(),
        vegld_price: value.vegld_price.toFixed(),
    }

	console.log('Settings: ', decoded);
}


(async function() {
	await account.sync(provider);
	await main();
})();