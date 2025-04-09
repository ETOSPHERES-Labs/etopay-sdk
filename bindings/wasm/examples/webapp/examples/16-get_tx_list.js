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

    let txs = await sdk.getTransactionList(start, limit);
    console.log("Transactions: " + JSON.stringify(txs));
}

export { main }
