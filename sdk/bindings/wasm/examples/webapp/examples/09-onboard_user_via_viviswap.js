import { initSdk } from './util';
import * as wasm from "cryptpay-sdk-wasm";

async function main() {
    let username = "satoshi";
    let mail = "satoshi@etogruppe.com";
    let termsAccepted = true;

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    let is_verified = await sdk.isKycVerified(username);
    console.log(`is verified: ${isVerified}`);
    if (is_verified) {
        console.log("User is already verified. No need to delete. Exiting");
        return;
    }

    // Create sap customer if not exists
    try {
        await sdk.getCustomer();
        console.log("sap customer exists. Continue");
    } catch (error) {
        await sdk.createCustomer("DE");
        console.log("created new sap customer");
    }

    // Start KYC verification for viviswap
    // The user already exists in viviswap db. Therefore, the test will fail here.
    const newUser = await sdk.startKycVerificationForViviswap("wasmtest@gmail.com", true);
    console.log(`New viviswap user: ${newUser}`);
}

export { main }
