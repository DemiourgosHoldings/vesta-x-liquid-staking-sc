PROXY=https://devnet-gateway.multiversx.com
CHAIN_ID="D"
WALLET="./wallets/shard1-test2.pem"
ADDRESS=$(mxpy data load --key=address-devnet)
######################################################################

DEPLOY_GAS_LIMIT=120000000

deploy() {
    mxpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-devnet.interaction.json" \
    --gas-limit=${DEPLOY_GAS_LIMIT}
    
    ADDRESS=$(mxpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    mxpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    mxpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --gas-limit=${DEPLOY_GAS_LIMIT}
}