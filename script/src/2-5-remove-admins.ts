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
import { ADMIN_ADDRESSES } from "./config";


async function main() {
	const args: TypedValue[] = [];
	ADMIN_ADDRESSES.map(value => args.push(new AddressValue(new Address(value))));
	
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`removeAdmins@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();