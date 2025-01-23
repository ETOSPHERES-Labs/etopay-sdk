# Release Notes

## [0.12.1] - 2024-06-27

### 🐛 Bug Fixes

- Api currency backwards compatibility

## [0.12.0] - 2024-06-19

### 🚀 Features

- Publish swift bindings package
- Add function to initialize a wallet from a mnemonic without stronghold

### 🐛 Bug Fixes

- Handle case id not found
- Update txs details query page param

### 🚜 Refactor

- Seprate create_new_wallet from init_wallet
- *(sdk)* Split viviswap with feature flags
- Adding xml for generating pom and setting xml files

### 📚 Documentation

- Update transactions validator docs
- Update kyc service docs
- Update user data service docs
- Update postident service docs
- Update webhook service docs
- Update main readme

### 🧪 Testing

- Add tests for the webhook-aggregator

### ⚙️ Miscellaneous Tasks

- Add commit sha for android sdk version
- *(docs)* Generate changelog from conventional commits
- Add testing crate
- Added deploy for wasm web target
- Bump `neon` to 1.0.0 for nodejs
- Bump version to 1.3.2 for dev container added kubectl and k9s
- Update pipeline to cache only on cargo-check push
- Updating toml files storing dependency in alphabetically sorted order
- Use rules of upstream pipeline
- *(release)* Bump version to 0.12
- *(test)* Fix conflicting tx using different account for test

## [0.11.0] - 2024-08-06

### 🚀 Features

- Allow multiple DLT nodes in SDK config
- Publish android SDK to Jfrog MVN
- Use array and Option in JNI bindings

### 🐛 Bug Fixes

- Admin service - add proper JSON format handling for send_event command
- Satoshi not kyc verified issue
- Don't return error when block does not have transaction payload
- Added resiliency against dapr when starting services

### 🚜 Refactor

- Move errors from common crate into services
- Remove duplicate client implementations from common
- Replace TxCreated event with grpc call

### ⚙️ Miscellaneous Tasks

- Update opentelemetry

## [v0.10.4]

- Added `get_wallet_transaction_list` - Returns the list of transactions of wallet by taking page and page size as a parameter.
- Added `get_wallet_transaction` - Returns the detailed report of particular transaction by taking transaction id as a parameter.

## [v0.10.0]

- The objects returned by `getTxList` now also include the exchange rate used for each transaction in the new field "course"
- Pagination of `getTxList` now works correctly.
- The functions `setCurrency` (introduced in v0.9.0), `getSwapList` and `getSwapDetails` (introduced in v0.8.2) had issues with the binding due to naming and could not be called from android, that has now been fixed and the functions are callable as intended.
- Added a check so that a transaction cannot be committed until it is marked as “Valid” by the backend. This partially fixes the issue with being able to give more compliments than the daily limit.

## [v0.9.0]

- Added new function `setCurrency`. From this version, this method will handle(setNodeUrl, setCoinType, setBech32Hrp) operations. For this reason those methods have been removed.
- Added new function `deleteUser. This method will allow a user to request to delete all of his data.

## [v0.8.0]

- Added `getSwapList`. Retrieves the list of all the swaps(viviswap orders) performed by a user
- Added `getSwapDetails`. Retrieves details for a specific swap(order in viviswap)

## [v0.7.3]

- Updated `get_iban` function. It now gives is_verified flag to show if the IBAN is verified by viviswap or not.
- The `createPurchaseRequest` function now requires the string literal "CLIK" as the string value in parameter `purchase_type` for compliments instead of "COMPLIMENT".
- Improved balance function internally to find hidden balances.

## [v0.7.1]

- Viviswap onboarding stabilized
- Production environment auth provider string was fixed (cawaenaprod → cawaena)
- `UpdateIBAN` method now deletes old IBAN and inserts the new one, so that the old IBAN can be re-used on other accounts

## [v0.7.0]

- Added new function `updateCustomer` which updates or creates a new customer account for billing. It requires the two-character
 country code of the user, which can be the same as the one used while doing a new user registration. Calling this function is mandatory before initializing wallet. The list of valid/legally allowed country codes will be given to the team before go-live.
- Modified the function createPurchaseRequest. It now requires three more parameters: `product_hash`, `app_data` (may be empty but can be used to store additional data of the image like thumbnail URL etc…), `purchase_type` (Required currently and should be set to the constant value “COMPLIMENT”)
- `getBalance` is now modified internally. It performs claiming of dust outputs, so that micro-transactions and locked outputs do not get lost. if not claimed. This feature is only internal to the crypto-currency and has no impact on the UX/UI.
- QA and staging environment now use SMR mainnet. Getting tokens now happens via either purchase from viviswap or transfer of existing SMR from own wallet-
- Webhook notifications can now be encrypted.

## [v0.6.4]

- Added new function `getTxList` which gives list of purchases and supports pagination. It gives the transaction list of all the transaction sent and received. If the value of the “receiver” is same as the username, the transaction was received, if different, then the transaction was sent.

## [v0.6.3]

- Added new optional function `setSdkEnv` which sets up the environment correctly according to predefined values, instead of setting up config individually. Individual config settings are still possible.
- Fixed issues related to compliments not working
- Fixed reported issues in capacitor plugin wrapper methods
- Staging environment now uses SMR mainnet. Getting tokens now happens via either purchase from viviswap or transfer of existing SMR from own wallet.
- The IOTA cryptocurrency is no longer MIOTA. Renamed to IOTA overall

## [v0.6.2]

- Modified `createDepositWithViviswap` function to accept no arguments any more.
- Modified `createWithdrawWithViviswap` requirements

## [v0.6.1]

- Modified `initLogger` function. Now no parameters are required. It however requires at least `setPathPrefix` and `setLogLevel`. Now returns error if logger cannot be initialized.
- Added `createViviswapDetail` function needed to perform withdrawals. It does not take any parameters.
- Modified `createWithdrawalWithViviswap` to now accept pin. If a pin is given, it tries to automatically perform withdrawal and also provides details for withdrawal. If no pin is given, it will just create a detail (address and an id) for manual withdrawal.
- Fixed issues with deposit and withdrawal at viviswap

## [v0.6.0]

- Added two new configuration settings `setCoinType` and `setBech32Hrp` which are now mandatory.
- Added helper function `validateConfig` for checking if configuration is correct. Currently it performs only syntactical validation and not semantical.
- Modified viviswap deposit and withdraw functions to now use the `coin type` set in the config to correctly create payment details.
- Added `getExchangeRate` function to fetch the latest exchange rate. The exchange rate considers the `coin type` set in the config with EUR for the course.
- Sdk will support MIOTA along with SMR after stardust upgrade on Mainnet (04.10.2023)

## [v0.5.2]

- Fixed issue in swift code and iOS bindings to make error responses exact to android bindings
- Migrated to iota-sdk from now deprecated wallet.rs

## [v0.5.0]

- Switched to SMR wallet. No changes in the SDK interface/functions
- Using JammDB instead of RocksDB to resolve issues with iOS SDK

## [v0.4.0]

- The functions `setAuthProvider`, `setPathPrefix`, `setBackendUrl` are not optional anymore.
- Switched to IOTA wallet. No change in the SDK interface/functions.
- Added new function setNodeUrl to set the IOTA Node URL (can be used with Mainnet as well)
- iOS SDK Class now 1-to-1 with Java SDK Class definition.

## [v0.3.0]

- Added viviswap functions for updating IBAN, creating deposit and withdrawal details
- Fixed a bug where user IOTA address was not getting updated in backend. This lead to failure in purchaseRequest flow. This is now fixed and it requires that all KYC verified users at least call `generateNewIotaReceiverAddress` before receiving and sending compliments.
- iOS now gives similar error descriptions to android
- Wallet backup and delete functions now end gracefully by dropping all open handles.
- Swift Plugin is now functionally same to Java.
