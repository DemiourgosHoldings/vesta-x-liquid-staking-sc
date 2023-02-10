import {
	Address,
	GasLimit,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	BooleanValue,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
    SET_SETTINGS_GAS_LIMIT,
	MANAGEMENT_ACTION_ALLOWED,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	const args: TypedValue[] = [
		new BooleanValue(MANAGEMENT_ACTION_ALLOWED),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setManagementActionAllowed@${argumentsString}`);

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