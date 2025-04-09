import * as wasm from "../pkg/etopay_sdk_wasm";
import { initSdk, PIN } from './utils';

async function main() {
    let username = "archiveme";

    const sdk = await initSdk(username);
    let password = "correcthorsebatterystaple";
    let mnemonic: string = (process.env.MNEMONIC as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(PIN, password);
    await sdk.createWalletFromMnemonic(PIN, mnemonic);
    await sdk.deleteUser(PIN);
    console.log("user deleted_successfully");
}

export { main }
