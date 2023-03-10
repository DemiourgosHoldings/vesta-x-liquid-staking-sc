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
	DefaultSmartContractController,
	CodeMetadata,
	BooleanValue,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
	SMART_CONRACT_ABI_URL,
	SMART_CONRACT_NAME,
    ESDT_MODULE_ADDRESS,
    TOKEN_ISSUE_COST,
    UNBONDING_PERIOD,
    TREASURY_WALLET,
    FEE,
    SET_SETTINGS_GAS_LIMIT,
	ADMIN_ADDRESSES,
	USER_ACTION_ALLOWED,
	MANAGEMENT_ACTION_ALLOWED,
	DELEGATE_ADDRESS,
	UNDELEGATE_ADDRESS,
	WHITELISTED_SP_ADDRESSES,
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
} from './util';
import { loadContractCode } from '@elrondnetwork/erdjs/out/testutils';

async function main() {
	const args: TypedValue[] = [];
	WHITELISTED_SP_ADDRESSES.map(row => args.push(new AddressValue(new Address(row))));
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`removeWhitelistedStakingProviderAddresses@${argumentsString}`);

	const tx = new Transaction({
		nonce: account.getNonceThenIncrement(),
		receiver: new Address(SMART_CONRACT_ADDRESS),
		data: data,
		gasLimit: new GasLimit(SET_SETTINGS_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}


(async function() {
	await account.sync(provider);
	await main();
})();