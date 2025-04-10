import * as wasm from "etopay-sdk-wasm";
import { initSdk, PIN } from './util';

async function main() {

    let username = "satoshi";
    let password = "correcthorsebatterystaple";

    const sdk = await initSdk();
    let mnemonic = process.env.MNEMONIC;
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);
    console.log("Wallet initialized!");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let address = await sdk.generateNewAddress(PIN);
    console.log("Address:", address);
}

export { main }
