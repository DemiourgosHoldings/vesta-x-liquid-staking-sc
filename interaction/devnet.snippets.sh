PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WALLET="./wallets/shard1-wallet.pem"
ADDRESS=$(erdpy data load --key=address-devnet)
######################################################################

DELEGATE_ADDRESS="erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzhllllsp9wvyl"
DELEGATE_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${DELEGATE_ADDRESS})"

###
ISSUE_COST=50000000000000000 # 0.05 EGLD

STAKE_AMOUNT=1000000000000000000
UNSTAKE_AMOUNT=1000000000000000000

VALAR_IDENTIFIER_ONLY_HEX="56414c41522d366239363238"
DATA_UNSTAKE_ONE_VALAR="ESDTTransfer@${VALAR_IDENTIFIER_ONLY_HEX}@0de0b6b3a7640000@756e7374616b65"

CALLER_ADDRESS="erd1ygdttzrulwfspme2s4qrx5y2qyfqalju0k2vcyy8z3979whlj9qssl5uay"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

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
    --value ${ISSUE_COST}
}

setDelegateAddress() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="setDelegateAddress" \
    --arguments ${DELEGATE_ADDRESS_HEX}
}

stake() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="stake" \
    --value ${STAKE_AMOUNT}
}

unstakeOneValar() {
    erdpy --verbose tx new --receiver ${ADDRESS} --recall-nonce --pem=${WALLET} --send --proxy=${PROXY} --chain=${CHAIN_ID} \
    --gas-limit=30000000 \
    --data=${DATA_UNSTAKE_ONE_VALAR}
}

claim() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=6000000 \
    --function="claim"
}

adminRedelegateRewards() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="adminRedelegateRewards"
}

adminWithdraw() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="adminWithdraw"
}

adminClaimRewards() {
    erdpy --verbose contract call ${ADDRESS} --send --proxy=${PROXY} --chain=${CHAIN_ID} --recall-nonce --pem=${WALLET} \
    --gas-limit=30000000 \
    --function="adminClaimRewards"
}

###

getValarIdentifier() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getValarIdentifier"
}

getDelegateAddress() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getDelegateAddress"
}

getValarSupply() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getValarSupply"
}

getStakedEgldAmount() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getStakedEgldAmount"
}

getUnbondingEgldAmount() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUnbondingEgldAmount"
}

getUnbondingUsers() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUnbondingUsers"
}

getUnbondingEgldAmountPerUser() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUnbondingEgldAmountPerUser" --arguments ${CALLER_ADDRESS_HEX}
}

getUnbondedEgldAmountPerUser() {
    erdpy --verbose contract query ${ADDRESS} --proxy=${PROXY} --function="getUnbondedEgldAmountPerUser"
}