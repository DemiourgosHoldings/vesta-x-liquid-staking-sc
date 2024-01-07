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
import { DELEGATE_ADDRESS } from "./config";

async function main() {
	const args: TypedValue[] = [
		new AddressValue(new Address(DELEGATE_ADDRESS)),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`claimRewardsFromStakingProvider@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();