
import { ADMIN_ADDRESSES } from './config';
import {
	provider,
	getSmartContract,
} from './provider';
import { Address, AddressValue, ResultsParser, TypedValue } from "@multiversx/sdk-core/out";

async function main() {
	try {
		const args: TypedValue[] = [
			new AddressValue(new Address(ADMIN_ADDRESSES[0])),
		];

		const contract = await getSmartContract();
		const interaction = contract.methodsExplicit.getUnstakingPacks(args);
		const query = interaction.check().buildQuery();
		const queryResponse = await provider.queryContract(query);
		const endpointDefinition = interaction.getEndpoint();
		const { firstValue, returnCode, returnMessage } =
			new ResultsParser().parseQueryResponse(queryResponse, endpointDefinition);

		if (!firstValue || !returnCode.isSuccess()) {
			throw Error(returnMessage);
		}

		const value = firstValue.valueOf();
		const decodeds = value.map(value => ({
			amount: value.amount.toFixed(),
			timestamp: value.timestamp.toNumber(),
		}));

		console.log('getUnstakingPacks: ', JSON.stringify(decodeds, null, 4));
	} catch (err) {
		console.log(err);
	}
}


(async function () {
	await main();
})();