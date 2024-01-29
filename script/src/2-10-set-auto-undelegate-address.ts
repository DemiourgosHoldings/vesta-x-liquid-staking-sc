import {
	Address,
	AddressValue,
	ArgSerializer,
	TransactionPayload,
	TypedValue,
  } from "@multiversx/sdk-core";
  import {
	UNDELEGATE_ADDRESS,
  } from "./config";
  import {
	createAndSendTransaction,
  } from './provider';

async function main() {
	const args: TypedValue[] = [
		new AddressValue(new Address(UNDELEGATE_ADDRESS)),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setAutoUndelegateAddress@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();