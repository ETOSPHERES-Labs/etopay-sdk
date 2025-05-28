import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username);
    let password = "correcthorsebatterystaple";
    let mnemonic: string = (process.env.MNEMONIC as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    await sdk.createNewWallet(PIN);
    console.log("Wallet initialized!");
    let address = await sdk.generateNewAddress(PIN);
    console.log("First Address:", address);

    // update the account and index to get another address
    await sdk.setWalletAccount(0, 1);
    let address2 = await sdk.generateNewAddress(PIN);
    console.log("Second Address:", address2);

}

main();

