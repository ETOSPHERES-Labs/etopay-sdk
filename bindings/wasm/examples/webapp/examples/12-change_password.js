import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";

    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;
    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    let new_password = "new_correcthorsebatterystaple"
    await sdk.setWalletPassword(PIN, new_password);
    console.log("change password successful");

    // fetch networks from backend
    let _ = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    // use wallet
    let _address = await sdk.generateNewAddress(PIN);
}

export { main }
