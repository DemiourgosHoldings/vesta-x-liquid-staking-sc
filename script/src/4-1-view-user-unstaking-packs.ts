import {
	provider,
	getSmartContract,
} from './provider';
import { ResultsParser } from "@multiversx/sdk-core/out";

async function main() {
	try {
		const contract = await getSmartContract();
		const interaction = contract.methodsExplicit.viewUserUnstakingPacks();
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
			address: value.address.toString(),
			packs: value.packs.map(v => ({
				amount: v.amount.toFixed(),
				timestamp: v.timestamp.toNumber(),
			})),
		}));

		console.log('viewUserUnstakingPacks: ', JSON.stringify(decodeds, null, 4));
	} catch (err) {
		console.log(err);
	}
}


(async function () {
	await main();
})();