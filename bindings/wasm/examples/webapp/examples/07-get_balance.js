import * as wasm from "etopay-sdk-wasm";
import { initSdk, PIN } from './util';

async function main() {
    console.log("start");
    const sdk = await initSdk();
    let username = "satoshi";
    let password = "correcthorsebatterystaple";

    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    // fetch networks from backend
    let _ = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    let address = await sdk.generateNewAddress(PIN);
    console.log("Address:", address);


    let balance = await sdk.getWalletBalance(PIN);
    console.log("Balance:", balance);

}

export { main }
