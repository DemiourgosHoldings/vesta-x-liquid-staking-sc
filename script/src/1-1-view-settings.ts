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
        const interaction = contract.methodsExplicit.viewLiquidStakingSettings();
        const query = interaction.check().buildQuery();
        const queryResponse = await provider.queryContract(query);
        const endpointDefinition = interaction.getEndpoint();
        const { firstValue, returnCode, returnMessage } =
            new ResultsParser().parseQueryResponse(queryResponse, endpointDefinition);

        if (!firstValue || !returnCode.isSuccess()) {
            throw Error(returnMessage);
        }

        const value = firstValue.valueOf();
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
    } catch (err) {
        console.log(err);
    }
}

(async function () {
    await main();
})();