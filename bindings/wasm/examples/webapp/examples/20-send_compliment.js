import * as wasm from "etopay-sdk-wasm";
import { sleep, initSdk, PIN } from "./util";

async function main() {
    console.log("start");
    const sdk = await initSdk();

    let username = "alice";
    let password = "correcthorsebatterystaple";

    let mnemonic = process.env.MNEMONIC;

    // ===========================================================================
    // Step 1: Initialize SDK for sender, create new user and wallet
    // ===========================================================================

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.initWalletFromMnemonic(PIN, mnemonic);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    let address = await sdk.generateNewAddress(PIN);
    console.log("Address:", address);

    let balance = await sdk.getWalletBalance(PIN);
    console.log("Balance:", balance);

    // ===========================================================================
    // Step 3: Create purchase request
    // ===========================================================================

    let product_hash = "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824";
    let app_data = "{\"imageUrl\":\"https://httpbin.org/\",\"imageId\":\"a846ad10-fc69-4b22-b442-5dd740ace361\"}";
    let purchase_type = "CLIK";

    let purchase_id = await sdk.createPurchaseRequest("satoshi", 2, product_hash, app_data, purchase_type);
    console.log("Purchase Request created: ", purchase_id);

    console.log("Waiting for purchase request to be valid:")
    while (true) {
        await sleep(5000);

        let details = await sdk.getPurchaseDetails(purchase_id);
        console.log("Details: ", details.toJSON());

        if (details.status == wasm.TxStatus.Valid) {
            console.log("Purchase request valid, moving on...");
            break;
        } else if (details.status == wasm.TxStatus.Invalid) {
            console.log("Purchase request invalid, exiting!");
            return
        }

    }


    // ===========================================================================
    // Step 4: Confirm purchase request (perform actual wallet transaction)
    // ===========================================================================

    console.log("Confirming purchase request...");
    await sdk.confirmPurchaseRequest(PIN, purchase_id);
    console.log("Confirming purchase request done, monitoring the status for 60 seconds or until it is completed:");
    for (let i = 0; i < 6; i++) {
        await sleep(10000);

        let details = await sdk.getPurchaseDetails(purchase_id);
        console.log("Status: ", details.toJSON());

        if (details.status == wasm.TxStatus.Completed) {
            console.log("Purchase request completed, done!");
            break;
        }
    }
    let new_balance = await sdk.getWalletBalance(PIN);
    console.log("New Balance:", new_balance);

}

export { main }
