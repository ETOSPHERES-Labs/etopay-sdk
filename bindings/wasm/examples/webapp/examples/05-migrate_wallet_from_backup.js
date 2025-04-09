import * as wasm from "etopay-sdk-wasm";
import { initSdk, PIN } from './util';

async function main() {
    let username = "satoshi";
    let password = "correcthorsebatterystaple";

    const sdk = await initSdk();
    let mnemonic = process.env.MNEMONIC;

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);

    await sdk.createWalletFromMnemonic(PIN, mnemonic);

    // Create wallet backup and delete it
    let backup_password = "backup_password";

    let backup = await sdk.createWalletBackup(PIN, backup_password);
    await sdk.deleteWallet(PIN)

    // Migrate wallet from backup
    await sdk.createWalletFromBackup(PIN, backup, backup_password);

    // fetch networks from backend
    let networks = await sdk.getNetworks();
    // set the network configuration for the wallet
    sdk.setNetwork(networks[0].key);

    // use wallet
    let _address = await sdk.generateNewAddress(PIN);
}

export { main }
