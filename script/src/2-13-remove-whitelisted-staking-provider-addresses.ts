import {
	Address,
	AddressValue,
	ArgSerializer,
	TransactionPayload,
	TypedValue,
  } from "@multiversx/sdk-core";
  import {
	WHITELISTED_SP_ADDRESSES,
  } from "./config";
  import {
	createAndSendTransaction,
  } from './provider';

async function main() {
	const args: TypedValue[] = [];
	WHITELISTED_SP_ADDRESSES.map(row => args.push(new AddressValue(new Address(row))));
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`removeWhitelistedStakingProviderAddresses@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();