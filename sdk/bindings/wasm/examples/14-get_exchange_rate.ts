import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk } from './utils';

async function main() {

    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    await sdk.getNetworks();
    sdk.setNetwork("67a1f08edf55756bae21e7eb");

    let course = await sdk.getExchangeRate();
    console.log(course);
}

main();

