import * as wasm from "../pkg/cawaena_sdk_wasm";
import { initSdk } from './utils';

async function main() {
    let username = "satoshi";
    let pin = "1234";
    const sdk = await initSdk(username);
    let mnemonic: string = (process.env.MNEMONIC as string);
    let password: string = (process.env.PASSWORD as string);

    await sdk.createNewUser(username);
    await sdk.initializeUser(username);
    await sdk.setWalletPassword(pin, password);

    await sdk.createWalletFromMnemonic(pin, mnemonic);

    await sdk.getNetworks();
    sdk.setNetwork("67a1f08edf55756bae21e7eb");

    let recipient_address = await sdk.generateNewAddress(pin);
    console.log("address", recipient_address);

    let balance_before = await sdk.getWalletBalance(pin);
    console.log("balance before sending amount", balance_before);

	const data = new TextEncoder().encode("wasm example");
    await sdk.sendAmount(pin, recipient_address, 1.0, data);

    let balance_after = await sdk.getWalletBalance(pin);
    console.log("balance after sending amount", balance_after);

}

main();


