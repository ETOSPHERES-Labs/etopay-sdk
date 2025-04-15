import * as wasm from "etopay-sdk-wasm";
import { initSdk, PIN } from './util';

async function main() {
    let username = "satoshi";
    let password = "correcthorsebatterystaple";

    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    await sdk.createNewWallet(PIN);

    console.log("created new wallet");
}

export { main }
