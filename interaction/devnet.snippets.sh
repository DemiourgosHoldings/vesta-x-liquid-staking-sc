PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET="./wallets/shard1-odin.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
######################################################################

UNBONDING_PERIOD=14400 # 4 hours
TREASURY_WALLET="erd15936k9pw34xyzmcaupyn7lpr7f6p20q50h4wlgemxg7h9zasdfysmhg50z"
TREASURY_WALLET_HEX="0x$(erdpy wallet bech32 --decode ${TREASURY_WALLET})"
FEE=1500 # 15%

ADMIN_ADDRESS="erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay"
ADMIN_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${ADMIN_ADDRESS})"

AUTO_DELEGATE_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqx0llllsdx93z0" # staking agency
AUTO_DELEGATE_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${AUTO_DELEGATE_ADDRESS})"

###
ISSUE_COST=50000000000000000 # 0.05 EGLD

CALLER_ADDRESS="erd15936k9pw34xyzmcaupyn7lpr7f6p20q50h4wlgemxg7h9zasdfysmhg50z"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

deploy() {
    erdpy --verbose contract deploy  --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --outfile="deploy-devnet.interaction.json" \
    --metadata-payable \
    --gas-limit=100000000
    
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

upgrade() {
    erdpy --verbose contract upgrade ${ADDRESS} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=100000000
}

issueVegldAndSetAllRoles() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=80000000 \
    --function="issueVegldAndSetAllRoles" \
    --value ${ISSUE_COST}
}

setSettings() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setSettings" \
    --arguments ${UNBONDING_PERIOD} ${TREASURY_WALLET_HEX} ${FEE}
}

setUnbondingPeriod() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setUnbondingPeriod" \
    --arguments ${UNBONDING_PERIOD}
}

setUserActionAllowed() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setUserActionAllowed" \
    --arguments 1
}

setAdminActionAllowed() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setAdminActionAllowed" \
    --arguments 1
}

addAdmins() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="addAdmins" \
    --arguments ${ADMIN_ADDRESS_HEX}
}

setFee() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setFee" \
    --arguments ${FEE}
}

setTreasuryWallet() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setTreasuryWallet" \
    --arguments ${TREASURY_WALLET_HEX}
}

adminMoveTreasury() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="adminMoveTreasury"
}

setAutoDelegateAddress() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="setAutoDelegateAddress" \
    --arguments ${AUTO_DELEGATE_ADDRESS_HEX}
}

###

getVegldIdentifier() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getVegldIdentifier"
}