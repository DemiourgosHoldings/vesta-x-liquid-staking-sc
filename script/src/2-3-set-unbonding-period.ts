import {
	ArgSerializer,
	U64Value,
	TransactionPayload,
	TypedValue,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';
import { UNBONDING_PERIOD } from "./config";


async function main() {
	const args: TypedValue[] = [
		new U64Value(UNBONDING_PERIOD),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setUnbondingPeriod@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();