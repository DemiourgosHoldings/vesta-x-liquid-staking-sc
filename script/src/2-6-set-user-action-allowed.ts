import {
	ArgSerializer,
	TransactionPayload,
	TypedValue,
	BooleanValue,
  } from "@multiversx/sdk-core";
  import {
	USER_ACTION_ALLOWED,
  } from "./config";
  import {
	createAndSendTransaction,
  } from './provider';


async function main() {
	const args: TypedValue[] = [
		new BooleanValue(USER_ACTION_ALLOWED),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setUserActionAllowed@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();