import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";

    let password = "correcthorsebatterystaple";
    let mnemonic = process.env.MNEMONIC;
    let receiver = "alice";

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    // fetch networks from backend
    let _ = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    let address = await sdk.generateNewAddress(PIN);
    console.debug(`Generated new IOTA receiver address: ${address}`);

    let balance = await sdk.getWalletBalance(PIN);

    console.log("balance : ", balance);

    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = JSON.stringify({
        "imageUrl": "https://httpbin.org/",
        "imageId": "a846ad10-fc69-4b22-b442-5dd740ace361"
    });

    let purchase_type = "CLIK";

    let purchase_id = await sdk.createPurchaseRequest(receiver, 2.0, product_hash, app_data, purchase_type);
    console.log("Purchase ID:", purchase_id);


}

export { main }
