import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username); // Initialize the SDK
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.WALLET_PASSWORD as string);

    await sdk.createNewUser(username); //Creating a new user 
    await sdk.initializeUser(username); // Initialize the user
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic); // Initialize the wallet
    console.log("Wallet initialized!");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let transactions = await sdk.getWalletTransactionList(PIN, 0, 10);  // Get the transaction list
    console.log("Wallet transactions list: " + JSON.stringify(transactions));
}

main();
