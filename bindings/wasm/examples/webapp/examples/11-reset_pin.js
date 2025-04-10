import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";

    let new_pin = "543216";
    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;
    const sdk = await initSdk();

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    await sdk.resetPin(PIN, new_pin);
    console.log("Reset PIN successfully");

    await sdk.verifyPin(new_pin);
    console.log("new PIN verified");
}

export { main }
