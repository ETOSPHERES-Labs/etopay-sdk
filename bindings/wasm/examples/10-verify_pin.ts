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

    await sdk.verifyPin(PIN);
    console.log("Pin verified successfully");

}

main();


