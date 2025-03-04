import * as wasm from "../pkg/cryptpay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let new_pin = "54321";
    const sdk = await initSdk(username);
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.PASSWORD as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    let new_password = "new password"
    await sdk.setWalletPassword(pin, new_password);
    console.log("change password successful");

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

main();

