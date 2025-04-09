import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    let new_pin = "543216";
    const sdk = await initSdk(username);
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.WALLET_PASSWORD as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    await sdk.resetPin(PIN, new_pin);
    console.log("Reset PIN successful");

    await sdk.verifyPin(new_pin);
    console.log("new PIN verified");
}

main();

