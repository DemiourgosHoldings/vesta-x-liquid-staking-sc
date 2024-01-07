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

async function main() {
	// const args: TypedValue[] = [
	// 	new AddressValue(new Address(DELEGATE_ADDRESS)),
  //       // new BigUIntValue(''),	// undelegate amount
	// ];
	// const { argumentsString } = new ArgSerializer().valuesToString(args);
	// const data = new TransactionPayload(`adminUndelegate@${argumentsString}`);
	const data = new TransactionPayload('undelegateFromStakingProvider');

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();