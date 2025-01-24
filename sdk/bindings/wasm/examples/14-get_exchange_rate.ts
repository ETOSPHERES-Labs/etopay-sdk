import * as wasm from "../pkg/cryptpay_sdk_wasm";
import { initSdk } from './utils';

async function main() {

    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    let course = await sdk.getExchangeRate();
    console.log(course);
}

main();

