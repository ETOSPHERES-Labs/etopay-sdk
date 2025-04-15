import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username);
    let password = "correcthorsebatterystaple";
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    let mnemonic = await sdk.createNewWallet(PIN);
    console.log("Mnemonic: ", mnemonic);
}

main();
