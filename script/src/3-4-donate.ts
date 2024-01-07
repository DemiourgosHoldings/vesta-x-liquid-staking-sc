import {
	TransactionPayload,
  } from "@multiversx/sdk-core";
  import {
	createAndSendTransaction,
  } from './provider';

async function main() {
	const data = new TransactionPayload(`donate`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();
