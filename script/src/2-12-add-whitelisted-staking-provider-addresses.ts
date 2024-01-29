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
import { WHITELISTED_SP_ADDRESSES } from "./config";

async function main() {
	const args: TypedValue[] = [];
	WHITELISTED_SP_ADDRESSES.map(row => args.push(new AddressValue(new Address(row))));
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`addWhitelistedStakingProviderAddresses@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();
