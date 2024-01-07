import {
	TransactionPayload,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';

async function main() {
	// const args: TypedValue[] = [
	// 	new AddressValue(new Address(DELEGATE_ADDRESS)),
  //       // new BigUIntValue(''),	// delegate amount
	// ];
	// const { argumentsString } = new ArgSerializer().valuesToString(args);
	// const data = new TransactionPayload(`adminDelegate@${argumentsString}`);
	const data = new TransactionPayload('delegateToStakingProvider');

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();