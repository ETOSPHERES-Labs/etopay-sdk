import { initSdk } from './util';
import * as wasm from "cryptpay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let pin = "1234";
    let password = "StrongP@55w0rd";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.setPassword(pin, password);
    await sdk.createWalletFromMnemonic(pin, mnemonic);
    let recipient_address = await sdk.generateNewAddress(pin);

    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(pin);

    console.log("balance before sending amount : ", balance_before);
    await sdk.sendAmount(pin, recipient_address, 1.0);
    let balance_after = await sdk.getWalletBalance(pin);
    console.log("balance after sending amount : ", balance_after);
}

export { main }


