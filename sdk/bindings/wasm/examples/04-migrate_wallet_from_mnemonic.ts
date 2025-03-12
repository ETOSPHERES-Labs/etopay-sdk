import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk, IOTA_NETWORK_ID } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    let password: string = (process.env.PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);

    // Create new wallet from the mnemonic
    await sdk.createWalletFromMnemonic(pin, mnemonic);

    // fetch networks from backend
    await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(IOTA_NETWORK_ID);

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

main();
