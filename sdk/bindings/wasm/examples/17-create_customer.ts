import * as wasm from "../pkg/cryptpay_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const country_code = "DE";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    try {
        await sdk.getCustomer(); // Get customer details
        console.log("Customer already exists.");
    } catch (error) {
        console.log(error);
        await sdk.createCustomer(country_code);
        console.log("user created successfully");
    }

}

main();

