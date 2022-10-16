PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET="./wallets/shard1-wallet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
######################################################################

VALAR_DISPLAY_NAME="Valar"
VALAR_DISPLAY_NAME_HEX="0x$(echo -n ${VALAR_DISPLAY_NAME} | xxd -p -u | tr -d '\n')"
VALAR_TICKER="VALAR"
VALAR_TICKER_HEX="0x$(echo -n ${VALAR_TICKER} | xxd -p -u | tr -d '\n')"
NUM_DECIMALS=18


###
ISSUE_COST=50000000000000000

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-devnet.interaction.json" \
    --metadata-payable \
    --gas-limit=60000000
    
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=60000000
}

issueValarAndSetAllRoles() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=60000000 \
    --function="issueValarAndSetAllRoles" \
    --value ${ISSUE_COST} \
    --arguments ${VALAR_DISPLAY_NAME_HEX} ${VALAR_TICKER_HEX} ${NUM_DECIMALS}
}


###

getValarIdentifier() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getValarIdentifier"
}