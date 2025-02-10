# SDK Internals

The Cawaena SDK is built in `rust`. It is primarily an implementation of the various interfaces for managing users, wallets, on-boarding of users through KYC (Know Your Customer) processes, payment methods and listing usage information.

The SDK was designed to support the `Cawaena` application. It is a social media application, which allows monetization of user-generated content. However, in the same principle, any digital data, given that it is authentic and its origin can be verified, can be monetized using the Cawaena ecosystem, which includes the Cawaena infrastructure and the sdk.

The big picture behind Cawaena is a data marketplace. Data processing, silo management and search engine features have been excluded by design from Cawaena to make it a minimal ecosystem for monetization.

## Overview of the SDK functional components

The figure below shows the functional component diagram of the SDK. The core of the SDK is a web3 hot-wallet. This wallet is used to store assets on the local machine running the application built with the SDK. The supporting components like the backend API, user state management and access control logic work for improving ease of use for the end user as well as ensuring correct process flow and state transitions between the Cawaena infrastructure and application.

The binding layer is just a simple 1-to-1 wrapper around the SDK interface functionality. This just exports the existing business logic implemented in the SDK in rust to other programming stacks to avoid re-implementation as well as guarantee memory safety natively in code.

The access control section at the bottom shows the input parameters needed from the user/application to authenticate itself against the SDK. For the one-time on-boarding in addition to the `pin` and `access_token` the `username` and `password` is also needed. For regular usage, the `pin`, whenever required and `access_token` is required to ensure smooth handling of operations, including internal function calls to the Cawaena infrastructure and the wallet.

```
+-------------------------------------------------------------------------------+
|                                                                               |
|   +-----------------------------------------------------------------------+   |
|   |                                                                       |   |
|   |                                                                       |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |   |                                             |    |              | |   |
|   |   |   +-------------------------------------+   |    |   Backend    | |   |
|   |   |   |                                     |   |    |   API        | |   |
|   |   |   |     +------------------------+      |   |    |              | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |                        |      |   |                     |   |
|   |   |   |     |       IOTA SDK         |      |   |    +--------------+ |   |
|   |   |   |     |                        |      |   |    |              | |   |
|   |   |   |     |       Stronghold       |      |   |    | User         | |   |
|   |   |   |     |         wallet         |      |   |    | State        | |   |
|   |   |   |     |         manager        |      |   |    | Management   | |   |
|   |   |   |     |                        |      |   |    +--------------+ |   |
|   |   |   |     +------------------------+      |   |                     |   |
|   |   |   |                                     |   |                     |   |
|   |   |   |           Wallet Manager            |   |    +--------------+ |   |
|   |   |   +-------------------------------------+   |    |              | |   |
|   |   |                                             |    |   Access     | |   |
|   |   |                Wallet User                  |    |   Control    | |   |
|   |   |                                             |    |   Logic      | |   |
|   |   +---------------------------------------------+    +--------------+ |   |
|   |                         SDK                                           |   |
|   +-----------------------------------------------------------------------+   |
|                           Bindings                                            |
|                                                                               |
|                                                                               |
+--+--------------------------------------+------+----------------------------+-+
   |      Onboarding authentication       |      |   Usage authentication     |  
   |                                      |      |                            |  
   +--^---------^-------^---------^-------+      +-----^-----------^----------+  
      |         |       |         |                    |           |             
      |         |       |         |                    |           |             
      |         |       |         |                    |           |             
   Username  Password  Pin    Access token            Pin      Access token      
```

## Understanding android bindings

The `libc++_shared.so` files are packaged together with the generated `*.so` files from cargo. These files are required by the corresponding system binary loader to correctly load the*.so files from the jniLibs folder using the convention

```Java
System.loadLibrary("library_name");
```

The library should be named with the prefix `lib` and the suffix `.so`. This is how the libraries are created by cargo when specifying the `crate-type` as `cdylib`. If the `libc++_shared.so` files for the corresponding architecture are not included, loading the library works a bit differently, however this method is not portable across different platforms.

```Java
System.load("absolute/path/to/lib/with/library_name.so");

```

### Creating a java wrapper

To support integration in java via a wrapper class, it is important to create the native methods in java and wrap them through a public method inside the class.

The convention followed by JNI is crucial here. The convention followed for declaring function names which are JNI compatible is: Java_{TLD}_{Org_Name}_{Class_name}_{function_Name}
where TLD = Top Level Domain
Org_Name = Organization name
Class_Name = Name of the class, whose method is private static and native.
function_Name = Name of the function

Example:
In Java, for the function `setCurrencyJni`, the following code is needed

```Java
// filename: CryptpaySdk.java
package com.etogruppe; // com is the TLD and etogruppe is the org_name

public class CryptpaySdk { // Class_name
    static {
        System.loadLibrary("walletsdk");
    }
    private static native void setCurrencyJni(String currency); // function_Name
    // other native functions here

    public void setCurrency(String currency) throws Exception {
        setCurrencyJni(currency);
    }

}

```

This would translate to a rust function as per the rule, with `no_mangle`, to tell the rust compiler to not mangle the function name in the resulting binary.

One point to note is not to use `snake_case` names of functions, because the JNI would deduce it as class path. `lowerCamelCase` or `CamelCase` is allowed.

```Rust
// lib.rs
#[no_mangle]
pub extern "system" fn Java_com_etogruppe_CryptpaySdk_setCurrencyJni(mut env: JNIEnv<'local>,
    _class: JClass<'local>) {
      // implementation
    }
```

The two variables are passed by the JNI call to rust, namely the JNIEnv and the java class. The JClass is an empty place holders to satisfy the Java interface. The JNI environment is a constant pointer the JNI environment and can be used to call methods for checking and raising exceptions as shown below.

```Rust
// lib.rs
fn on_error_null(error: SdkError, env: &mut JNIEnv) {
    // fearlessly doing unwraps here, because we are already in exception
    if env.exception_check().unwrap() {
        // exception was raised,  clear it and raise again
        // to return back to JVM
        env.exception_clear().unwrap();
    }
    env.throw(format!("{error:#?}")).unwrap();
}
```

It can also be used to get strings and other objects from the java native to rust safely and pass back rust strings as java string objects back. These methods are needed almost on every function call and return, and hence have been written as macros to reduce code repetitions.

```Rust
// lib.rs
macro_rules! get_jni_string {
    ($env:ident, $on_error:ident, $input_string:expr) => {{
        let jni_string = match $env.get_string(&$input_string) {
            Ok(jni_string) => String::from(jni_string),
            Err(err) => {
                let error = SdkError::UnhandledError(format!("Invalid string input: {err:#}"));
                $on_error(error, &mut $env);
                String::from("")
            }
        };
        jni_string
    }};
}

macro_rules! set_jni_string {
    ($env:expr, $string:expr) => {
        $env.new_string($string).unwrap()
    };
}

```

### Futures across the ABI boundary in Java

As noticed in the previous example of the rust function, the prefix `extern "system"` is used  to tell compiler some information on how the function is going to be called. The `extern` helps the compiler to know that the function will be called from outside of rust. The `system` describes the application binary interface (ABI) to use to define the function in binary code. With `system` the systems ABI is to be used. This ABI is defined by each system according to the target-triplet configured during the cargo build command.

As we will see later, we could also use `C` ABI to expose our functions using the `C` calling conventions in the binary interface. Based on the ABI selected, different applications running on the same host machine and using the same operating system can interact with each other through the binary layer for exchanging information or calling functions.

Unfortunately, on the binary layer there is no easy concept of futures, since there is no scheduler, which can periodically check if a result of a certain function call is available. Calling an external application function from another application is generally synchronous and the result is returned immediately. This means that for asynchronous function calls like HTTP requests, a blocking [sync] call has to be used by the external application and the calling application has to treat the function call as synchronous.

For the Cawaena SDK, there is a runtime thread started and used for calling the SDK functions, in this case, are all asynchronous. Using `OnceCell` this runtime thread is created on the first call to any SDK function and reused by all others.

```Rust
// lib.rs
fn runtime() -> &'static Arc<Runtime> {
    static INSTANCE: OnceCell<Arc<Runtime>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .thread_name("com.standalone.sdk.thread")
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
        )
    })
}
```

This results in a function definition without need of async await in the exposed interface, since currently there are no known ABI's that support this. The future is actively blocked on, thus making it synchronous.

```Rust
// lib.rs
// async/await is not allowed in the function definition here. Is there any ABI that could do it?
#[no_mangle]
pub extern "system" fn Java_com_etogruppe_CryptpaySdk_setPathPrefix<'local>( 
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    path_prefix: JString<'local>,
) {
    let path_prefix = get_jni_string!(env, on_error_null, path_prefix);

    runtime().block_on(async move {
      // async/await can be done here as it is inside a thread which is "blocked on" i.e. synchronous
        let mut sdk = get_or_init_sdk().write().await;
        sdk.set_path_prefix(&path_prefix)
    });
}
```

A nice way to solve this on the other side would be to wrap the native ABI calls in an interface which uses the futures framework of the used programming language (if available). So, the java class would now look something like this:

```Java
// filename: CryptpaySdk.java
package com.etogruppe; 
import java.util.concurrent.Future; // get the future support from the language

public class CryptpaySdk { 
    static {
        System.loadLibrary("walletsdk");
    }
    private static native void setCurrencyJni(String currency); 
    
    // exposed class method now gives a future wrapped void and not just void!
    public Future<void> setCurrency(String currency) throws Exception {
        setCurrencyJni(currency);
    }
}
```

Wrapping the exposed interface with a future, tells the application which builds on top of this external library function, that it is indeed asynchronous and can implement mechanisms to handle it correspondingly.

## Understanding Swift bindings

### Generating iOS xcframeworks

The iOS ecosystem allows developers to create xcframeworks, which are a distribution format that contains multiple architectures and platforms in a single bundle. The xcframeworks offer advantages such as improved performance, reduced app size, and easier distribution and integration.

Our xcframework structure is as following:

- CryptpaySdkBin.xcframework/
  - ios-arm64/
    - Headers/
      - generated/
        - Cawaena-sdk-swift-new/
          - Cawaena-sdk-swift-new.h
        - SwiftBridgeCore.h
      - bridging-header.h
      - module.modulemap
    - libwalletsdk_cabi_new.a
  - macos-arm64_x86_64/
    - Headers/
      - generated/
        - Cawaena-sdk-swift-new/
          - Cawaena-sdk-swift-new.h
        - SwiftBridgeCore.h
      - bridging-header.h
      - module.modulemap
    - libwalletsdk_cabi_new.a

The creation can be done using the `xcodebuild` command, which takes the static library as input and an include section, which is actually a folder containing at most one module.modulemap file and any other C header files which describe the interface to the static library, assuming that it is C ABI.

The module map should include all the header files for working with the static library and re-export them as shown below.

```module.modulemap
module CryptpaySdkBin {
    header "bridging-header.h"
    export *
}
```

The module name is important here. It needs to have the name of the package binary which in our case is `CryptpaySdkBin`.

The `bridging-header.h` file includes the auto-generated files from the `swift-bridge` crate which we explain in more details on the `header files` section below.

The `lipo` utility is used to re-arrange the Rust generated static library with binary information. It allows developers to combine multiple architectures into a single binary file.

Installing an xcframework in the iOS app development environment is simply done by dragging and dropping the xcframework file in xcode or importing it using the settings menu.

### Swift package

Previously we were bundling our Swift bindings into a `.zip` file and exporting it. This was not very practical for the users and also for us since we need to bundle, download it and give it to our users manually. Instead, we want to offer it as a package. In this version we generate a Swift package instead which can be imported as a dependency. The structure of our Swift package is as following:

- CryptpaySdk/
  - CryptpaySdkBin.xcframework/
  - Sources/
    - CryptpaySdk/
      - SwiftBridgeCore.swift
      - Cawaena-sdk-swift-new.swift
      - vectorizable.swift
  - Package.swift

The `CryptpaySdkBin.xcframework` creation and its internal structure is explained in the above `Generating iOS xcframeworks` section. The `SwiftBridgeCore.swift` and `Cawaena-sdk-swift-new.swift` files are auto-generated by the `swift-bridge` crate.  

The `vectorizable.swift` contains custom implementations for Rust types which cannot be bridged directly to Swift. For example the `Vec<KycOpenDocument` type needs to implement the `Vectorizable` to compile in Swift. This is due to some limitations on the crate which need some workaround to compile properly.  

Finally, the `Package.swift` is the actual file that generates the Swift package called `CryptpaySdk`. In xcode we can add the CryptpaySdk packge and initialize the constructor and call its functions (our Rust bindings):  

```Swift
  import CryptpaySdk
  let sdk = CryptpaySdk()
  try await sdk.setEnvironment("development")
```

### Swift-Bridge

We are using the [`swift-bridge`](https://crates.io/crates/swift-bridge) crate to generate the Swift bindings.

#### Built-in Types

The `swift-bridge` crate uses the `#[swift_bridge::bridge]` procedural macro to declare a bridge module. This macro parses the bridge module at compile time and then generates the Rust side of the FFI layer. The bridge module is declared inside the `lib.rs` module.

```Rust
#[swift_bridge::bridge]
pub mod ffi {
    pub enum UserState {
        Undefined,
        New,
        Verified,
        Locked,
        Unlocked,
    }

    extern "Rust" {
        #[swift_bridge(swift_name = "getUserState")]
        async fn get_user_state(&self) -> Result<UserState, String>;
      }
}
```

For the above function it will generate the following C ABI in the backgroud, which will make the bridge with Swift possible:

```Rust
pub extern "C" fn __swift_bridge__CryptpaySdk_get_user_state(callback_wrapper: *mut std::ffi::c_void, callback: extern "C" fn(*mut std::ffi::c_void, ResultUserStateAndString), this: *mut super::CryptpaySdk)
```

In the `java` bindings we serialize any complex return values into JSON and return a `String`. We also take enum variants as `Strings` and perform conversion internally.  

The `swift-bridge` crate supports sharing standard library types between Rust and Swift. We can return `Result` or `Option` types in the response which can contain a complex type such as a `struct` or `enum`. These types can ba used in the response and also as function arguments. In our case, we always return a `Result` type and on the `Err` case we are returning a formatted `String` of the internal SDK error.

The swift-bridge allows this if we declare shared types between Rust and Swift in the bridge module as shown in the example above with the `UserState` enum.

Additionally, we should write the conversion between these types. In our case we have developed a helper macro for generating the `From` implementations for enums and structs. In other cases we do the conversion manually, depending on the use case and limitations.

```Rust
convert_enum!(
    sdk::types::users::UserState,
    ffi::UserState,
    Undefined,
    New,
    Verified,
    Locked,
    Unlocked,
);
```

There are still some limitations with this library to passing some complex types and require some adaptations to work out. For example, the `Option<String>` type is not supported in return structs. In this case, the workaround would be to use `.unwrap_or("".to_string())` on the Option value. It attempts to unwrap the `Option` that comes from the SDK and get the `String` value. Otherwise, if the unwrap fails, meaning we get a `None` value, it returns an empty `String` which would be equivalent to the `None` value. For more information regarding the supported build-in types refer to the official [documentation]("https://chinedufn.github.io/swift-bridge/built-in/index.html") of `swift-bridge` crate.  

#### async support

As we can see from the example above, the `swift-bridge` library supports `async/await` functions between Swift and Rust. Calling an async Rust function from Swift is supported. Calling an async Swift function from Rust is not yet supported. In our case, we are exporting async functions from Rust to Swift, which is supported. Therefore we do not need to embedd a runtime (such as `tokio`) ourselves!

At build time, on the background, it runs `swift-bridge-build` on files that contain bridge modules in order to generate the Swift and C code necessary to make the bridge work.

#### header files

The `swift-bridge` crate along with `build.rs` is used to create the header files from the source code directly. Although one could write these header files by hand, but for a SDK with many functions, it makes sense to automate it for avoiding errors and missing out any functions.

The `build.rs` links Swift to the Rust native library. Aside the header files, it generates also the Swift files.  

It parses the Rust source files for `#[swift_bridge::bridge]` procedural macro headers and generates the corresponding Swift files. It then writes the auto-generated files to a single Swift file and all of the generated C headers to a single header file. These files are generated under the `/include` folder so that it can become a part of the xcframework without moving or copying it.  

This is what the auto-generated header files structure look like:  

- generated/
  - Cawaena-sdk-swift-new/
    - Cawaena-sdk-swift-new.h
    - Cawaena-sdk-swift-new.swift
  - SwiftBridgeCore.h
  - SwiftBridgeCore.swift

## Hot and Cold wallets

Users today maintain their crypto-currencies in wallets. A wallet is purely software coupled with a secret store, from which the addresses are deterministically derived from a single seed using hierarchical deterministic procedure. For more information, see [BIP-0032](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki). The actual tokens/coins of the cryptocurrency reside on these address. Thus, in reality, the wallet is a method of accessing and controlling the crypto-currency, which in turn always lies on the network and never in a hardware or in the typical sense inside a wallet.

Since, the wallet has two components, the software and the storage, this allows to classify the wallet based on how these components are implemented and in which environment, specifically under whose control, these components are running/installed. The wallet matrix is shown as below:

| Environment      | Software       | Storage     |
|------------------|----------------|-------------|
| User/Individual  | Cold/Hot-Wallet| Non-custodial |
| Exchange/Business| Hot-Wallet     | Custodial   |

Users use cold non-custodial wallets to keep access and control over their secret seeds, which effectively give them control over their keys. Some users also use hot custodial wallets to efficiently trade cryptocurrencies at exchanges and participate in various decentralized finance (DeFi) protocols like lending pools, swaps, staking, etc.

A well-informed and researched user will temporarily maintain hot custodial wallets to engage with the chain and market and permanently maintain a major chunk of funds to addresses controlled by the non-custodial cold wallet. The user will then shuffle between the two wallets based on funds risk.

### Hot Wallets: The Swift Side of Crypto

Picture a hot wallet as the bustling city centre of your digital finances. Hot wallets are online, connected to the internet, and readily available for transactions. They provide users with quick access to their cryptocurrencies, making them ideal for active trading and daily transactions. Think of them as your go-to pocket wallet for everyday spending in the digital realm.

However, convenience comes at a cost. The very connectivity that makes hot wallets user-friendly also renders them more vulnerable to cyber threats. Hacking attempts and online attacks pose a constant risk, making it crucial for users to exercise caution and implement additional security measures when relying on hot wallets.

### Cold Wallets: Embracing the Fortress of Security

Now, shift your focus to the serene fortress nestled away from the hustle and bustle – the cold wallet. Unlike their hot counterparts, cold wallets remain offline and disconnected from the internet. This deliberate isolation provides a higher level of security, shielding your digital assets from the prying eyes of online threats.

Cold wallets are the guardians of large sums of cryptocurrency, often employed for long-term storage. While they may not offer the immediacy of hot wallets, their offline nature makes them an attractive option for those prioritizing the safety and longevity of their crypto investments.

## The IOTA wallet

The wallet used within the SDK is the official wallet developed by the IOTA Foundation and maintained in its own SDK found [here](https://github.com/iotaledger/iota-sdk). The wallet internally uses the stronghold secret management engine also developed by the IOTA Foundation found [here](https://github.com/iotaledger/stronghold.rs). The secret management engine not only stores sensitive data in files but also uses obfuscation and mechanisms against memory dumps to protect the secrets while they are being operated upon in the memory. Stronghold also provides functions for BIP-0032 derivation using the BIP-0044 derivation path mechanism described [here](https://github.com/bitcoin/bips/blob/master/bip-0044.mediawiki). The word list used by the wallet is the word list described in BIP-0039 [here](https://raw.githubusercontent.com/bitcoin/bips/master/bip-0039/english.txt).

The various coin types supported by BIP-0044 can be found in the list [here](https://github.com/satoshilabs/slips/blob/master/slip-0044.md). Both `IOTA` and `SMR` are supported and have the coin types `4218` and `4219` respectively.

Currently, in its base implementation the IOTA SDK also needs an in-memory key-value store to manage some metadata related to the stronghold engine and other wallet settings. The IOTA SDK uses a rocksdb implementation in rust for this purpose. There are a few noteworthy problems with rocksdb:

- rocksdb is not light-weight for mobile end devices and the resulting binaries of the sdk take long to build and are bigger in storage requirements.
- rocksdb does not support all mobile platforms
- rocksdb is not maintained on the latest sdks of the android and iOS mobile platforms

After investigation, it was found that the in-memory key-value store was used only for storing some metadata keys and not necessarily need high-performance query execution. Luckily, the IOTA SDK implemented the rocksdb connection as a `Storage` trait. Since, the SDK already used jammdb for its internal key-value store, a fork was created and the trait was implemented using `jammdb`. A pull request was created to the upstream, but the dev team at IOTA Foundation recommended to maintain the fork for now, as there would be some new breaking changes coming and the pull request can be created at a later point. The fork is updated regularly and maintained [here](https://github.com/mighty840/iota-sdk).

## Pin and password in the SDK

Generally, the password requirements for any application need to meet today's standards. This might become difficult for the user to remember their wallet stronghold password and also an irritating experience to enter it every time even for the smallest of transactions. On the other side, for a secure wallet application, the SDK should not rely on the interfacing application to do password management for a secret manager used internally. This has a lot of side effects, such as, the application might bypass the SDK logic for protecting access to the secret by simply using the password against the file, with no knowledge of the SDK. This is a security risk and cannot be accepted.

The end devices today support pin entry mostly protected by biometric authentication for ease but secure user experience, when it comes to accessing a restricted OS functionality. Taking all this in account, the SDK was designed to provide the end users possibilities to set up their wallet using a password and a pin.

The password stays with the SDK in an encrypted form and only the pin can be used to decrypt it. Thus, for every operation with the secret manager, where a password is needed, the user must only enter the pin, or allow the application to fetch it through bio-metrically protected secure storages on end devices. This solves the problem of user experience.

The issue of password management is also solved, since now the SDK internally manages the password, while still relying completely on the user to unblock it using the pin. The SDK cannot act in its own interest even if there was a malicious code trying to unblock the wallet! The probability distribution of the pin, being relatively weak, (4 to 6 digit), is improved through the addition of a pseudo random salt, which in combination with a hash function results in an encryption password of significant strength and quasi-random probability distribution. This is used then to encrypt the password for the secret manager.

Thus an attacker would need information on the salt, the encrypted password, pin and the stronghold file to be able to gain access to the wallet functions. This is tough and would need somehow physical access to the end device, and to the end user. Security of end-user and their devices is out of the scope for Cawaena ecosystem.

## Vertical spreads

A vertical spread in the context of exchange rates between two currencies for a crypto exchange refers to the price difference (spread) between the bid and ask prices of a particular cryptocurrency pair.

In a vertical spread, the bid price represents the highest price that a buyer is willing to pay for a specific cryptocurrency, while the ask price represents the lowest price at which a seller is willing to sell the same cryptocurrency. The vertical spread is the numerical difference between these two prices.

For example, let's say the bid price for Bitcoin (BTC) against US Dollar (USD) is $50,000 and the ask price is $50,100. The vertical spread in this case would be $100 ($50,100 - $50,000).

Vertical spreads are significant for traders and investors because they indicate liquidity and market depth. A narrow spread suggests a highly liquid market with many buyers and sellers, whereas a wide spread may indicate lower liquidity and potentially higher transaction costs. Traders often look for tight vertical spreads when executing trades to minimize costs and ensure efficient transactions.

If an exchange offers no vertical spread, it means that the bid and ask prices for a particular cryptocurrency pair are identical or extremely close to each other. Essentially, there is no difference between the highest price a buyer is willing to pay (bid price) and the lowest price a seller is willing to accept (ask price).

Having no vertical spread implies high liquidity and efficiency in the market. It indicates that there are many buyers and sellers actively trading the cryptocurrency pair, resulting in competitive pricing and minimal transaction costs for traders.

Exchanges that offer no vertical spread are highly desirable for traders because they allow for instant execution of trades at fair market prices without incurring significant costs associated with spreads. This can contribute to a smoother trading experience and better opportunities for traders to enter and exit positions efficiently.

Viviswap takes the burden of the vertical spread on itself by ensuring highest liquidity always! (Tough to achieve)
