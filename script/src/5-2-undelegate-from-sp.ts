import * as fs from 'fs';
import {
	Address,
	GasLimit,
	TransactionPayload,
	Transaction,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
	ADMIN_UNDELEGATE_GAS_LIMIT,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	// const args: TypedValue[] = [
	// 	new AddressValue(new Address(DELEGATE_ADDRESS)),
  //       // new BigUIntValue(''),	// undelegate amount
	// ];
	// const { argumentsString } = new ArgSerializer().valuesToString(args);
	// const data = new TransactionPayload(`adminUndelegate@${argumentsString}`);
	const data = new TransactionPayload('undelegateFromStakingProvider');

	const tx = new Transaction({
		nonce: account.getNonceThenIncrement(),
		receiver: new Address(SMART_CONRACT_ADDRESS),
		data: data,
		gasLimit: new GasLimit(ADMIN_UNDELEGATE_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}


(async function() {
	await account.sync(provider);
	await main();
})();