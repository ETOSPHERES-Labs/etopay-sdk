# ETOPay SDK – WASM Bindings

This package provides ***WebAssembly (WASM)*** bindings for the ETOPay SDK, enabling seamless integration with both _Web_ applications and _NodeJS_ environments. It generates TypeScript type definitions for all relevant data objects, ensuring type safety and ease of use. Additionally, this package includes functionality for publishing both the WASM binary and the generated bindings to an NPM registry.

### 🚀 Quickstart

For a hands-on example of how to integrate the ETOPay SDK, check out our public quickstart guide:
👉 [ETOPay SDK Quickstart (TypeScript)](https://github.com/ETOSPHERES-Labs/etopay-sdk-quickstart-ts).

### 📖 Usage

Before integrating the ETOPay SDK, you must create a project via our [ETOPay Dashboard](https://etopayapp.etospheres.com). Once registered, you will receive some _credentials_, which are required for setting up and using the SDK.

You can find more information regarding our products on our [ETOSPHERES](https://etospheres.com/) website.

### 📦 Installation

Install the ETOPay SDK via bun:

```bash
bun install @etospheres/etopay-sdk-wasm
```
❗ Currently, we only support [`bun`](https://bun.sh/) for running the SDK, as `node` does not work correctly with the current version. We recommend using [`bun`](https://bun.sh/) to ensure proper functionality.

### 🛠️ Example Usage

You can check out more examples in our [public docs](https://docs.etospheres.com/SDK%20Examples/Examples/).