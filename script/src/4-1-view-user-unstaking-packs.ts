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
	const interaction = contractInteractor.contract.methods.viewUserUnstakingPacks();
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
        console.log('res', res);
        return;
    }
	const values = res.firstValue.valueOf();
	
    const decodeds = values.map(value => ({
        address: value.address.toString(),
        packs: value.packs.map(v => ({
			amount: v.amount.toFixed(),
			timestamp: v.timestamp.toNumber(),
		})),
    }));

	console.log('viewUserUnstakingPacks: ', JSON.stringify(decodeds, null, 4));
}


(async function() {
	await account.sync(provider);
	await main();
})();