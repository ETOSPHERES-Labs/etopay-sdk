import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "satoshi";

    const sdk = await initSdk(username);
    let password = "correcthorsebatterystaple";
    let mnemonic: string = (process.env.MNEMONIC as string);
    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    // Create new wallet from the mnemonic
    await sdk.createNewWallet(PIN);

    // Create wallet backup and delete it
    let backup_password = "backup_password";

    let backup = await sdk.createWalletBackup(PIN, backup_password);
    await sdk.deleteWallet(PIN)

    // Migrate wallet from backup
    await sdk.createWalletFromBackup(PIN, backup, backup_password);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork("iota_rebased_testnet");

    // use wallet
    let _address = await sdk.generateNewAddress(PIN);
}

main();
