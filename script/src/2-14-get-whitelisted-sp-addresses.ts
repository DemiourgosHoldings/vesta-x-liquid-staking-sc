import {
	account,
	provider,
	getSmartContractInteractor,
} from './provider';

async function main() {
	const contractInteractor = await getSmartContractInteractor();
	const interaction = contractInteractor.contract.methods.getWhitelistedStakingProviderAddresses();
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
        console.log('res', res);
        return;
    }
	const values = res.firstValue.valueOf();
    const decoded = values.map(v => v.toString());

	console.log('getWhitelistedStakingProviderAddresses: ', decoded);
}

(async function() {
	await account.sync(provider);
	await main();
})();