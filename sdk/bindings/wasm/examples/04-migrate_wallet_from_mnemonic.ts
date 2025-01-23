import * as wasm from "../pkg/cryptpay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    let password: string = (process.env.PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setPassword(pin, password);

    // Create new wallet from the mnemonic
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

main();
