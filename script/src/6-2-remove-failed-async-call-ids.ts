import {
	Address,
	GasLimit,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	U64Value,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
	ADMIN_CLAIM_REWARDS_GAS_LIMIT,
	FAILED_ASYNC_CALL_IDS,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	const args: TypedValue[] = [];
	FAILED_ASYNC_CALL_IDS.map(row => args.push(new U64Value(row)));
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`removeFailedAsyncCallIds@${argumentsString}`);

	const tx = new Transaction({
		nonce: account.getNonceThenIncrement(),
		receiver: new Address(SMART_CONRACT_ADDRESS),
		data: data,
		gasLimit: new GasLimit(ADMIN_CLAIM_REWARDS_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}

(async function() {
	await account.sync(provider);
	await main();
})();