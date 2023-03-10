import {
	ZERO_ADDRESS,
} from "./config";

import {
	account,
	provider,
	signer,
	getSmartContractInteractor,
} from './provider';
import BigNumber from 'bignumber.js';
import {
	sleep,
	convertWeiToEsdt,
	convertEsdtToWei,
} from './util';

async function main() {
	const contractInteractor = await getSmartContractInteractor();
	const interaction = contractInteractor.contract.methods.viewLiquidStakingSettings();
	const res = await contractInteractor.controller.query(interaction);

	if (!res || !res.returnCode.isSuccess()) {
        console.log('res', res);
        return;
    }
	const value = res.firstValue.valueOf();
	
    const decoded = {
        vegld_identifier: value.vegld_identifier.toString(),
        treasury_wallet: value.treasury_wallet.toString(),
        fee: value.fee.toNumber(),
        unbonding_period: value.unbonding_period.toNumber(),
        admins: value.admins.map(v => v.toString()),
        user_action_allowed: value.user_action_allowed,
        management_action_allowed: value.management_action_allowed,
        is_token_roles_set: value.is_token_roles_set,
		auto_delegate_address: value.auto_delegate_address.toString() == ZERO_ADDRESS ? '' : value.auto_delegate_address.toString(),
		auto_undelegate_address: value.auto_undelegate_address.toString() == ZERO_ADDRESS ? '' : value.auto_undelegate_address.toString(),

        pool_vegld_amount: value.pool_vegld_amount.toFixed(),
        pool_egld_amount: value.pool_egld_amount.toFixed(),
        prestaked_egld_amount: value.prestaked_egld_amount.toFixed(),
        preunstaked_egld_amount: value.preunstaked_egld_amount.toFixed(),
        unbonded_egld_amount: value.unbonded_egld_amount.toFixed(),
        pending_reward_egld_amount: value.pending_reward_egld_amount.toFixed(),

        vegld_price: value.vegld_price.toFixed(),
        failed_async_call_ids: value.failed_async_call_ids.map(v => v.toNumber()),
    }

	console.log('Settings: ', decoded);
}

(async function() {
	await account.sync(provider);
	await main();
})();