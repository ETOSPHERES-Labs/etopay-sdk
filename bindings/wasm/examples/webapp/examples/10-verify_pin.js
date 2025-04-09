import { initSdk } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    const sdk = await initSdk();
    let username = "satoshi";
    let pin = "123456";
    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    let is_verified = await sdk.verifyPin(pin);
    console.log("verification done successfully");
}

export { main }
