import {
	ArgSerializer,
	TransactionPayload,
	TypedValue,
	BigUIntValue,
	StringValue,
  } from "@multiversx/sdk-core";
  import {
	USER_UNSTAKE_AMOUNT,
	VEGLD_TOKEN,
  } from "./config";
  import {
	createAndSendTransaction,
  } from './provider';


async function main() {
	const args: TypedValue[] = [
		new StringValue(VEGLD_TOKEN),
		new BigUIntValue(USER_UNSTAKE_AMOUNT),
		new StringValue('userUnstake'),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`ESDTTransfer@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();