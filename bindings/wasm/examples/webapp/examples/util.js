import * as wasm from "etopay-sdk-wasm";
const sleep = (delay) => new Promise((resolve) => setTimeout(resolve, delay))

export { sleep }

export const PIN = "123456";

// Function to initialize the SDK
export async function initSdk() {

    console.log("Starting SDK initialization...");

    const sdk = new wasm.ETOPaySdk(); // Create an instance of the SDK

    const url = process.env.EXAMPLES_BACKEND_URL;
	await sdk.initLogger(wasm.Level.Trace);
    await sdk.setConfig(`
    {
        "backend_url": "${url}",
        "auth_provider": "standalone"
    }
    `);

    await sdk.refreshAccessToken(process.env.ACCESS_TOKEN);
    return sdk; // Return the initialized SDK instance
}
