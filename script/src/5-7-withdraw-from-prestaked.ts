import {
	Address,
	AddressValue,
	GasLimit,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
	DELEGATE_ADDRESS,
	ADMIN_CLAIM_REWARDS_GAS_LIMIT,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	const data = new TransactionPayload(`withdrawFromPrestaked`);

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