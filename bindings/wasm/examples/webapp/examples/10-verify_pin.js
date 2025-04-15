import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    const sdk = await initSdk();
    let username = "satoshi";

    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    let is_verified = await sdk.verifyPin(PIN);
    console.log("verification done successfully");
}

export { main }
