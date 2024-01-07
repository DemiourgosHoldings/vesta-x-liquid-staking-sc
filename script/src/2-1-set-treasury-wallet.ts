import {
	Address,
	AddressValue,
	ArgSerializer,
	TransactionPayload,
	TypedValue,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';
import { TREASURY_WALLET } from "./config";

async function main() {
	const args: TypedValue[] = [
		new AddressValue(new Address(TREASURY_WALLET)),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setTreasuryWallet@${argumentsString}`);

	await createAndSendTransaction(data);
}


(async function() {
	await main();
})();