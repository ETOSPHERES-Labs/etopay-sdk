import { initSdk } from './util';
import * as wasm from "cawaena-sdk-wasm";

async function main() {
    let username = "satoshi";
    const country_code = "DE";
    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    try {
        await sdk.getCustomer();
        console.log("customer already exists.");
    } catch (error) {
        console.log(error);
        await sdk.createCustomer(country_code);
        console.log("user created successfully");

    }
}

export { main }
