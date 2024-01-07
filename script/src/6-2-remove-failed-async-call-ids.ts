import {
	Address,
	AddressValue,
	ArgSerializer,
	U64Value,
	TransactionPayload,
	TypedValue,
	TokenIdentifierValue,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';
import { FAILED_ASYNC_CALL_IDS } from "./config";

async function main() {
	const args: TypedValue[] = [];
	FAILED_ASYNC_CALL_IDS.map(row => args.push(new U64Value(row)));
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`removeFailedAsyncCallIds@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();