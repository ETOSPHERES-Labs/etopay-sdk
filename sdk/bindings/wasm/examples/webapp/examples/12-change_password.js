import { initSdk } from './util';
import * as wasm from "cawaena-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;
    const sdk = await initSdk();

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

export { main }
