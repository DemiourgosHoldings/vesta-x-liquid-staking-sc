import {
	TransactionPayload,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';

async function main() {
	const data = new TransactionPayload(`userWithdraw`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();
