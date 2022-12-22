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
    USER_STAKE_AMOUNT,
    USER_STAKE_GAS_LIMIT,
	ADMIN_DELEGATE_GAS_LIMIT,
	DELEGATE_ADDRESS,
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

async function main() {
	const args: TypedValue[] = [
		new AddressValue(new Address(DELEGATE_ADDRESS)),
        // new BigUIntValue(''),	// delegate amount
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`adminDelegate@${argumentsString}`);

	const tx = new Transaction({
		nonce: account.getNonceThenIncrement(),
		receiver: new Address(SMART_CONRACT_ADDRESS),
		data: data,
		gasLimit: new GasLimit(ADMIN_DELEGATE_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}


(async function() {
	await account.sync(provider);
	await main();
})();