import {
	Address,
	GasLimit,
	Egld,
	TransactionPayload,
	Transaction,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
    TOKEN_ISSUE_COST,
    ISSUE_GAS_LIMIT,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	const data = new TransactionPayload(`issueVegldAndSetAllRoles`);

	const tx = new Transaction({
		nonce: account.getNonceThenIncrement(),
		receiver: new Address(SMART_CONRACT_ADDRESS),
		data: data,
        value: Egld.value(TOKEN_ISSUE_COST),
		gasLimit: new GasLimit(ISSUE_GAS_LIMIT),
	});

	await signer.sign(tx);
	const txHash = await tx.send(provider);
	console.log(`${EXPLORER_URL}${txHash.toString()}`);
}

(async function() {
	await account.sync(provider);
	await main();
})();