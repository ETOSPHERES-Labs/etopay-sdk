import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";

    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    // fetch networks from backend
    let _ = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    let recipient_address = await sdk.generateNewAddress(PIN);

    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(PIN);
    console.log("balance before sending amount : ", balance_before);

    let tx_id = await sdk.sendAmount(PIN, recipient_address, 1.0);
    console.log("sent amount with transaction", tx_id);

    let balance_after = await sdk.getWalletBalance(PIN);
    console.log("balance after sending amount : ", balance_after);
}

export { main }


