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

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let recipient_address = await sdk.generateNewAddress(PIN);
    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(PIN);
    console.log("balance before sending amount", balance_before);

    const data = new TextEncoder().encode("wasm example");
    let tx_id = await sdk.sendAmount(PIN, recipient_address, 1.0, data);
    console.log("sent amount with transaction", tx_id);

    let balance_after = await sdk.getWalletBalance(PIN);
    console.log("balance after sending amount", balance_after);

    let details = await sdk.getWalletTransaction(PIN, tx_id);
    console.log("transaction details: ", details);

}

main();


