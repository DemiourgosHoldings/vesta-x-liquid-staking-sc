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
	EVE_ADDRESS,
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
	const args: TypedValue[] = [
		new AddressValue(new Address(EVE_ADDRESS)),
	];

	const contractInteractor = await getSmartContractInteractor();
	const interaction = contractInteractor.contract.methods.getUnstakingPacks(args);
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
        console.log('res', res);
        return;
    }
	const values = res.firstValue.valueOf();
	
    const decodeds = values.map(value => ({
		amount: value.amount.toFixed(),
		timestamp: value.timestamp.toNumber(),
    }));

	console.log('getUnstakingPacks: ', JSON.stringify(decodeds, null, 4));
}


(async function() {
	await account.sync(provider);
	await main();
})();