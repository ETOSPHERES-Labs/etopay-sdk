import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username);
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password = "correcthorsebatterystaple";

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    let new_password = "new_correcthorsebatterystaple"
    await sdk.setWalletPassword(PIN, new_password);
    console.log("change password successful");

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let _address = await sdk.generateNewAddress(PIN);
}

main();

