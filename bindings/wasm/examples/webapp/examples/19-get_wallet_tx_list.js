import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let start = 0;
    let limit = 10;

    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);
    console.log("Wallet initialized!");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let txs = await sdk.getWalletTransactionList(PIN, start, limit);
    console.log("Wallet transactions list : " + JSON.stringify(txs));
}

export { main }
