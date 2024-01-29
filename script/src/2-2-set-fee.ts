import {
	ArgSerializer,
	U64Value,
	TransactionPayload,
	TypedValue,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';
import { FEE } from "./config";

async function main() {
	const args: TypedValue[] = [
		new U64Value(FEE),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setFee@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
	await main();
})();