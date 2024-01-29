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
	const data = new TransactionPayload(`withdrawFromPrestaked`);

	await createAndSendTransaction(data);
}

(async function() {
  await main();
})();