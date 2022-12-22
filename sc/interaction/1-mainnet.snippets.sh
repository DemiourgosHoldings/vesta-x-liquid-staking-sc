PROXY=https://gateway.elrond.com
CHAIN_ID="1"
WALLET="./wallets/vestax-vegld.pem"
ADDRESS=$(erdpy data load --key=address-mainnet)
######################################################################

DEPLOY_GAS_LIMIT=90000000

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-mainnet.interaction.json" \
    --gas-limit=${DEPLOY_GAS_LIMIT}
    
    ADDRESS=$(erdpy data parse --file="deploy-mainnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-mainnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --gas-limit=${DEPLOY_GAS_LIMIT}
}