PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"
WALLET="./wallets/eve.pem"
ADDRESS=$(erdpy data load --key=address-testnet)
######################################################################

DEPLOY_GAS_LIMIT=90000000

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-testnet.interaction.json" \
    --gas-limit=${DEPLOY_GAS_LIMIT}
    
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-testnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --gas-limit=${DEPLOY_GAS_LIMIT}
}