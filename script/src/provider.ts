import * as fs from 'fs';
import {
	AbiRegistry,
	Account, Address, SmartContract, Transaction, TransactionPayload,
  } from "@multiversx/sdk-core";
import {
	UserSigner, parseUserKey,
  } from "@multiversx/sdk-wallet";
import {
	GATEWAY_URL,
	PEM_PATH,
	EXPLORER_URL,
	CHAIN_ID,
	SMART_CONRACT_ABI_URL,
	SMART_CONRACT_NAME,
	SMART_CONRACT_ADDRESS,
	API_URL,
} from "./config";
import { ProxyNetworkProvider, ApiNetworkProvider } from '@multiversx/sdk-network-providers/out';
import { COMMON_CONFIG_GAS_LIMIT } from './config-devnet';

export const provider = new ProxyNetworkProvider(GATEWAY_URL, { timeout: 20000 });
export const apiProvider = new ApiNetworkProvider(API_URL, { timeout: 20_000 });
// let config = NetworkConfig.getDefault();
// config.ChainID = new ChainID(CHAIN_ID);
// config.sync(provider);

const pem = fs.readFileSync(PEM_PATH, { encoding: 'utf-8' }).trim();
export const signer = UserSigner.fromPem(pem);

export const getSmartContract = async () => {
  const abiJson = await fs.readFileSync(SMART_CONRACT_ABI_URL, {
    encoding: "utf8",
  });
  const abiObj = JSON.parse(abiJson);
  const abiRegistry = AbiRegistry.create(abiObj);
  return new SmartContract({
    address: new Address(SMART_CONRACT_ADDRESS),
    abi: abiRegistry,
  });
};

export const getAccount = async () => {
  const userKey = parseUserKey(pem);
  const address = userKey.generatePublicKey().toAddress();

  const account = new Account(address);
  const apiAccount = await apiProvider.getAccount(address);
  account.update(apiAccount);
  return account;
};

export const signAndSendTx = async (tx: Transaction) => {
  const serializedTransaction = tx.serializeForSigning();
  const signature = await signer.sign(serializedTransaction);
  tx.applySignature(signature);
  await provider.sendTransaction(tx);
  console.log(`${EXPLORER_URL}${tx.getHash().toString()}`);
} 

export const createAndSendTransaction = async (data: TransactionPayload) => {
  const account = await getAccount();
  
  const tx = new Transaction({
    sender: account.address,
    nonce: account.getNonceThenIncrement(),
    receiver: new Address(SMART_CONRACT_ADDRESS),
    data: data,
    chainID: CHAIN_ID,
    gasLimit: COMMON_CONFIG_GAS_LIMIT,
  });

  await signAndSendTx(tx);
}