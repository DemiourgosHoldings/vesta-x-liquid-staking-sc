import {
	ArgSerializer,
	TransactionPayload,
	TypedValue,
	BooleanValue,
  } from "@multiversx/sdk-core";
  import {
	  MANAGEMENT_ACTION_ALLOWED,
  } from "./config";
  import {
	createAndSendTransaction,
  } from './provider';


async function main() {
	const args: TypedValue[] = [
		new BooleanValue(MANAGEMENT_ACTION_ALLOWED),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setManagementActionAllowed@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();