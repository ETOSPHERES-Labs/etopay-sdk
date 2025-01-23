import * as wasm from "../pkg/cryptpay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    let is_verified = await sdk.isKycVerified(username)

    if (is_verified) {
        console.log("user is verified");
        return;
    }

    try {

        await sdk.getCustomer();
        console.log("sap customer exists. Continue");

    } catch (error) {

        await sdk.createCustomer("DE");
        console.log("created new sap customer");

    }

    // Start KYC verification for viviswap
    // The user already exists in viviswap db. Therefore, the test will fail here.

    let newUser = await sdk.startKycVerificationForViviswap("wasmtest@gmail.com", true);
    console.log(`New viviswap user: ${newUser}`);
}


main();


