import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk, IOTA_NETWORK_ID } from './utils';

async function main() {
    console.log("start");
    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    let password: string = (process.env.PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);

    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // fetch networks from backend
    await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(IOTA_NETWORK_ID);

    let address = await sdk.generateNewAddress(pin);
    console.log("Address:", address);

    let balance = await sdk.getWalletBalance(pin);
    console.log("Balance:", balance);

}

main();
