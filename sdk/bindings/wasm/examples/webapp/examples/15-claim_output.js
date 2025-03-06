import { initSdk } from './util';
import * as wasm from "cawaena-sdk-wasm";

async function main() {

    let username = "satoshi";
    let password = "StrongP@55w0rd";
    let pin = "1234";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    console.log("Wallet initialized!");

    await sdk.claimOutputs(pin);
    console.log("Outputs claimed!");
}

export { main }
