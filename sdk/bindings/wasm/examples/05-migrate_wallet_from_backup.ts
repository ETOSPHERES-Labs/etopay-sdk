import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk } from './utils';

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
    await sdk.createNewWallet(pin);

    // Create wallet backup and delete it
    let backup_password = "backup_password";

    let backup = await sdk.createWalletBackup(pin, backup_password);
    await sdk.deleteWallet(pin)

    // Migrate wallet from backup
    await sdk.createWalletFromBackup(pin, backup, backup_password);

    await sdk.getNetworks();
    sdk.setNetwork("67a1f08edf55756bae21e7eb");

    // use wallet
    let _address = await sdk.generateNewAddress(pin);
}

main();
