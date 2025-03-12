import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk, IOTA_NETWORK_ID } from './utils';

async function main() {

    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);

    // fetch networks from backend
    await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(IOTA_NETWORK_ID);

    let course = await sdk.getExchangeRate();
    console.log(course);
}

main();

