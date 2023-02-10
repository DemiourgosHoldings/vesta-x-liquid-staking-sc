import {
	Address,
	AddressValue,
	GasLimit,
	ArgSerializer,
	TransactionPayload,
	Transaction,
	TypedValue,
	U64Value,
	BooleanValue,
} from "@elrondnetwork/erdjs/out";
import {
	EXPLORER_URL,
	SMART_CONRACT_ADDRESS,
    UNBONDING_PERIOD,
    TREASURY_WALLET,
    FEE,
    SET_SETTINGS_GAS_LIMIT,
	USER_ACTION_ALLOWED,
	MANAGEMENT_ACTION_ALLOWED,
} from "./config";

import {
	account,
	provider,
	signer,
} from './provider';

async function main() {
	const args: TypedValue[] = [
		new U64Value(UNBONDING_PERIOD),
		new AddressValue(new Address(TREASURY_WALLET)),
        new U64Value(FEE),
		new BooleanValue(USER_ACTION_ALLOWED),
		new BooleanValue(MANAGEMENT_ACTION_ALLOWED),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setSettings@${argumentsString}`);

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