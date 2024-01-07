import {
	ZERO_ADDRESS,
} from "./config";

import {
	provider,
	getSmartContract,
} from './provider';
import { ResultsParser } from "@multiversx/sdk-core/out";

async function main() {
	try {
		const contract = await getSmartContract();
		const interaction = contract.methodsExplicit.getWhitelistedStakingProviderAddresses();
		const query = interaction.check().buildQuery();
		console.log('before provider');
		const queryResponse = await provider.queryContract(query);
		console.log('after provider');
		const endpointDefinition = interaction.getEndpoint();
		const { firstValue, returnCode, returnMessage } =
			new ResultsParser().parseQueryResponse(queryResponse, endpointDefinition);

		if (!firstValue || !returnCode.isSuccess()) {
			console.log('first');
			throw Error(returnMessage);
		}

		console.log('second');
		const value = firstValue.valueOf();
		const decoded = value.map(v => v.toString());

		console.log('getWhitelistedStakingProviderAddresses: ', decoded);
	} catch (err) {
		console.log(err);
	}
}

(async function () {
	await main();
})();