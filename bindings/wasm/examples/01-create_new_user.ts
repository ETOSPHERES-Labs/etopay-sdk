import * as wasm from "../pkg/etopay_sdk_wasm";// Import the WASM module
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    const sdk = await initSdk(username);
    await sdk.createNewUser(username);

    // Temporary
    let m = await sdk.printTime();
    console.log("printTime: ", m);
    let c = await sdk.debugConfig();
    console.log("debugConfig: ", c);
    let cc = await sdk.checkCollision();
    console.log("checkCollision: ", cc);

    console.log("user created successfully");

}

main();
