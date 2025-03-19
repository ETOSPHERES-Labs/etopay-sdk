# ETOPay SDK WASM bindings

In order to use the etopay sdk in a webapp the browser, this crate provides `wasm` bindings and generation of TypeScript type definitions for all the data type objects used.
It also has functionality for publishing the bindings together with the generated `wasm` binary to a NPM registry.

## Design considerations

### Making the `sdk` compile for `wasm`

The `wasm` target is special as it is executed in a JVM-like isolated virtual machine in the browser. Therefore, access to system resources such as files is not allowed unless using the existing JavaScript APIs to
access them through the browser. In the `sdk`, we store user state and wallet information in an on-disk database. This works fine for the native targets, but due to the limitations of `wasm` this is not possible when
compiling for the web. Thus, to even make the `sdk` compile for `wasm`, the usage of such resources is removed and replaced with in-memory alternatives.

This is the current setup:

- There are two feature flags in the `sdk` to gate the functionality that is not available on the `wasm` target:
  1. The 'stronghold' feature hides all functionality that uses the `stronghold` library in the `iota-sdk` (since it uses file-based storage).
  2. The 'jammdb_repo' feature enables the use of the on-disk `UserRepo` implementation using `jammdb`, otherwise an in-memory database is used.

- This binding crate depends on the `sdk` crate with `default-features = false`, which effectively disables the above features when compiling the binding crate _alone_
  (otherwise feature unification will enable the features again).

- This crate specifies the `wasm32-unknown-unknown` build target in a local `.cargo/config.toml` which is considered by `cargo` when building from within this crate's directory.

### The actual bindings
The bindings to JavaScript/TypeScript are generated using [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) which also generates TypeScript type definitions.
To ease development we use [`wasm-pack`](https://github.com/rustwasm/wasm-pack) to handle all the configuration and compiler flags as well as generation of a NPM package (see [Development](#development)).

In code, we export a top-level `ETOPaySdk` object  using the `#[wasm_bindgen]` macro which has all the functions attached like so:
```rust
use std::sync::Arc;
use tokio::sync::RwLock;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ETOPaySdk {
    inner: Arc<RwLock<Sdk>>,
}

#[wasm_bindgen]
impl ETOPaySdk {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // do some initialization
        let mut sdk = Sdk::default();
        sdk.initialize("parameter");

        Self {
            inner: Arc::new(RwLock::new(sdk)),
        }
    }

    /// Method exported on the `ETOPaySdk` object
    #[wasm_bindgen(js_name = "associatedFunction")]
    pub async fn associated_function(&self, parameter: String) {
        let mut sdk = self.inner.write().await;
        sdk.some_function(&parameter)
    }
}
```

from JavaScript/TypeScript code we can then do something like this (after importing the package with the name `wasm`)
```typescript
async function main() {
    // the function marked `constructor` is executed
    const sdk = new wasm.ETOPaySdk();

    await sdk.associatedFunction("value");
}
main();
```

Since the `sdk` does not provide any interior mutability by itself, we need to take care of that in the binding layer.
`wasm_bindgen` does support exporting functions that take `&mut self` as receiver and then internally uses a `RefCell` to
provide runtime checking of rusts aliasing rules (e.g. that there is only one mutable reference or many immutable reference to the `self`
object at any single point in time). Since this runtime checking could cause runtime panics in case the JavaScript code calls multiple functions
simultaneously (perhaps from a separate service worker), we opted to instead provide our own interior mutability
using the `tokio::sync::RwLock`, much like in the other bindings. This way we always take `&self` as receiver parameter
and will not run into any runtime panics. See [exporting a struct to js](https://rustwasm.github.io/wasm-bindgen/contributing/design/exporting-rust-struct.html?highlight=free#exporting-a-struct-to-js)
for more details on the internals of `wasm-bindgen`.

> Note that we do not need to use a `StaticCell` or similar to provide one-time initialization of the sdk since we directly
> embed the creation into the constructor of the `ETOPaySdk` object. Thus the lifetime is directly tied to the
> variable's lifetime in the JavaScript VM in the browser. `wasm-bindgen` also exposes a `free()` method which can be called
> to release the memory held by the exported object.

### Using rich `rust` types

In the `java` bindings we serialize any complex return values into JSON and return a `String`. We also take enum variants as `String`s and perform conversion internally.
With the use of `wasm-bindgen`, however, we can directly take `enum` arguments and return rust `struct`s by annotating them with the `#[wasm_bindgen]` attribute!
To avoid sprinkling the entire `sdk` (and sometimes even `api_types`) with these annotations, and to make it even clearer when we change a user-facing struct, we
opted to provide duplicate definitions of these types inside this binding crate, and perform conversion to/from the internal `sdk` types manually.
This adds some code duplication, but confines all `wasm` related code into the binding crate directly and we have the option to rename the generated types
if we want. The docstrings are also carried over to the generated TypeScript definitions!

`wasm-bindgen` also natively supports returning `Result`s, which translates to an exception being thrown upon returning the `Err` variant.


### A note on `async` support
`wasm-bindgen` does support `async` functions out-of-the-box and generates bindings that return `Promise<>` to the user. Therefore we do not need
to embed a runtime (such as `tokio`) ourselves! For this to work correctly, however, the async functions we use with the `async_trait` helper-macro
_must not have `Send` bounds_, see [wasm-bindgen#2409](https://github.com/rustwasm/wasm-bindgen/issues/2409 ). Luckily the `async_trait` crate
[supports this](https://github.com/dtolnay/async-trait/tree/master?tab=readme-ov-file#non-threadsafe-futures) and we can use
the `cfg_attr` attribute to only apply this when compiling for the `wasm32` target (since the `Send` bound is needed when using the `tokio` runtime):
```rust
use async_trait::async_trait;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TheTrait {
    ...
}

```


## Development

### Build with `wasm-pack build`

```
wasm-pack build
```

### Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### Publish to NPM with `wasm-pack publish` after building

```
wasm-pack publish
```

### Executing the example scripts
1. Navigate to examples directory: `cd sdk/bindings/wasm/examples`.
2. Add a `.env` file as shown below, with the corresponding values:
```
MNEMONIC=
MNEMONIC_ALICE=
KC_URL=
KC_REALM=
KC_CLIENT_ID=
KC_CLIENT_SECRET=
PASSWORD=
EXAMPLE_BACKEND_URL=
```
3. Install dependencies: `bun install` or `pnpm install`.
4. Build the package with `nodejs` as target: `wasm-pack build --target nodejs`.
5. Run individual examples, ex: `ts-node 07-get_balance.ts`. In case there is some error occuring from not finding `ts-node` command, use `npx ts-node 07-get_balance.ts`.

In case you want to run all the examples after installing the dependencies and building the package:
1. Navigate back to sdk directory: `cd sdk`.
2. Run: `make run_wasm_node_examples`. This will execute the `run_all_examples.sh` script on `sdk/bindings/wasm/examples`, which runs all the examples.

### Executing the webapp example

1. Follow step 1 above to make sure you have a `.env` file in the root with a valid access token for either `satoshi`, `archiveme` or `alice` depending on example which you want to run.(access token can be obtained with the `hello.http` API request in `docs/api-requests/hello.http`).
2. Go into the webapp folder and install the dependencies:
  ```bash
  cd examples/webapp
  pnpm install
  ```
3. Start the development server with `pnpm start`. This will start a server on port `8080`, which you can forward to the host (if vscode does not do this automatically)
   by going to the `PORTS` tab in the vscode terminal and adding port `8080`. Then you can view the webapp at `http://localhost:8080/`. Open the developer console (F12)
   to view the sdk logs. Click the button of the example you want to run.
