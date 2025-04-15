import { initSdk, PIN } from './util';
import * as wasm from "etopay-sdk-wasm";

async function main() {
    let username = "satoshi";

    const sdk = await initSdk();
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    let course = await sdk.getExchangeRate();
    console.log(course);
}

export { main }
