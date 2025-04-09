import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username);
    let password: string = (process.env.WALLET_PASSWORD as string);
    let mnemonic: string = (process.env.MNEMONIC as string);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    // Create new wallet from the mnemonic
    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    // use wallet
    let _address = await sdk.generateNewAddress(PIN);
}

main();
