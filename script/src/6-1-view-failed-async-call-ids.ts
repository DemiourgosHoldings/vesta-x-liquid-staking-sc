import {
	account,
	provider,
	getSmartContractInteractor,
} from './provider';

async function main() {
	const contractInteractor = await getSmartContractInteractor();
	const interaction = contractInteractor.contract.methods.viewFailedAsyncCallIds();
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
		console.log('res', res);
		return;
	}

	const values = res.firstValue.valueOf();
	const decodeds = values.map(value => value.toNumber());

	console.log('viewFailedAsyncCallIds: ', decodeds);
}

(async function() {
	await account.sync(provider);
	await main();
})();