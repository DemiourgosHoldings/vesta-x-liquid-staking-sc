import {
	Address,
	AddressValue,
	ArgSerializer,
	U64Value,
	TransactionPayload,
	TypedValue,
	BooleanValue,
  } from "@multiversx/sdk-core";
  import {
	  FEE,
	  MANAGEMENT_ACTION_ALLOWED,
	TREASURY_WALLET,
	UNBONDING_PERIOD,
	USER_ACTION_ALLOWED,
  } from "./config";
  import {
	  createAndSendTransaction,
  } from './provider';

async function main() {
	const args: TypedValue[] = [
		new U64Value(UNBONDING_PERIOD),
		new AddressValue(new Address(TREASURY_WALLET)),
        new U64Value(FEE),
		new BooleanValue(USER_ACTION_ALLOWED),
		new BooleanValue(MANAGEMENT_ACTION_ALLOWED),
	];
	const { argumentsString } = new ArgSerializer().valuesToString(args);
	const data = new TransactionPayload(`setSettings@${argumentsString}`);

	await createAndSendTransaction(data);
}

(async function() {
	await main();
})();