# Examples

The Cawaena SDK is built in `rust`. It is primarily an implementation of the various interfaces for managing users, wallets, on-boarding of users through KYC (Know Your Customer) processes, payment methods and listing usage information. The flows discussed in this document show examples using the rust language. For examples related to the specific language, refer to the corresponding pages.

The Cawaena SDK can be used only if the following pre-requisites have been fulfilled and the information resulting from these conditions is available:

- Oauth2.0 Identity and Access Management Provider is configured correctly,
- The backend base URL of the Cawaena system is known,
- The path to a certain file storage is available, where the application has read/write rights to create, modify and delete files related to the SDK like log files, local key-value DBs, and wallet files.

Once this information is available, the SDK can be instantiated and the basic functions can be used.

The examples shows the usage of the SDK in rust for creating a user. The user credentials are taken from the environment but could also be easily a user input.

The environment configuration to `Development` attaches the SDK to the development backend of Cawaena automatically. It also configures the authentication provider correctly with the one used by the development team internally. This configuration is used by Cawaena developers and is only restricted to the users controlled by the identity provider configured for Cawaena internal testing.

## 0. Shared Setup Code

=== "Rust"

    ```rust
    --8<-- "examples/utils.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/utils.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/utils.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/utils.ts"
    ```

## 1. Create New User

=== "Rust"

    ```rust
    --8<-- "examples/01_create_new_user.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/CreateNewUser01.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/01_create_new_user.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/01-create_new_user.ts"
    ```

## 2. Onboard User Postident

=== "Rust"

    ```rust
    --8<-- "examples/02_onboard_user_postident.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/OnboardUserPostident02.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/02_onboard_user_postident.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/02-onboard_user_via_postident.ts"
    ```

## 4. Migrate Wallet From Mnemonic

=== "Rust"

    ```rust
    --8<-- "examples/04_migrate_wallet_from_mnemonic.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/MigrateWalletFromMnemonic04.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/04_migrate_wallet_from_mnemonic.swift"
    ```

=== "JS/TS"

    N/A

## 5. Migrate Wallet From Backup

=== "Rust"

    ```rust
    --8<-- "examples/05_migrate_wallet_from_backup.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/MigrateWalletFromBackup05.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/05_migrate_wallet_from_backup.swift"
    ```

=== "JS/TS"

    N/A

## 6. Generate Iota Receiver Address

=== "Rust"

    ```rust
    --8<-- "examples/06_generate_new_iota_address.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GenerateNewIotaAddress06.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/06_generate_new_iota_address.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/06-generate_new_iota_address.ts"
    ```

## 7. Get Balance

=== "Rust"

    ```rust
    --8<-- "examples/07_get_balance.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetBalance07.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/07_get_balance.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/07-get_balance.ts"
    ```

## 8. Create Purchase Request

=== "Rust"

    ```rust
    --8<-- "examples/08_create_purchase_request.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/CreatePurchaseRequest08.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/08_create_purchase_request.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/08-create_purchase_request.ts"
    ```

## 9. Onboard a User on Viviswap

=== "Rust"

    ```rust
    --8<-- "examples/09_onboard_user_viviswap.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/OnboardUserViviswap09.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/09_onboard_user_viviswap.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/09-onboard_user_via_viviswap.ts"
    ```

## 10. Verify Pin

=== "Rust"

    ```rust
    --8<-- "examples/10_verify_pin.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/VerifyPin10.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/10_verify_pin.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/10-verify_pin.ts"
    ```

## 11. Reset Pin

=== "Rust"

    ```rust
    --8<-- "examples/11_reset_pin.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/ResetPin11.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/11_reset_pin.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/11-reset_pin.ts"
    ```

## 12. Change Password

=== "Rust"

    ```rust
    --8<-- "examples/12_change_password.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/ChangePassword12.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/12_change_password.swift"
    ```

=== "JS/TS"

    N/A

## 13. Send Amount

=== "Rust"

    ```rust
    --8<-- "examples/13_send_amount.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/SendAmount13.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/13_send_amount.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/13-send_amount.ts"
    ```

## 14. Get Exchange Rate

=== "Rust"

    ```rust
    --8<-- "examples/14_get_exchange_rate.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetExchangeRate14.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/14_get_exchange_rate.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/14-get_exchange_rate.ts"
    ```

## 15. Claim Outputs

=== "Rust"

    ```rust
    --8<-- "examples/15_claim_output.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/ClaimOutput15.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/15_claim_output.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/15-claim_output.ts"
    ```

## 16. Get Purchase List

=== "Rust"

    ```rust
    --8<-- "examples/16_get_tx_list.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetTxList16.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/16_get_tx_list.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/16-get_tx_list.ts"
    ```

## 17. Create Customer

=== "Rust"

    ```rust
    --8<-- "examples/17_create_customer.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/CreateCustomer17.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/17_create_customer.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/17-create_customer.ts"
    ```

## 18. Delete User

=== "Rust"

    ```rust
    --8<-- "examples/18_delete_user.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/DeleteUser18.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/18_delete_user.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/18-delete_user.ts"
    ```

## 19. Get Wallet Transaction List

=== "Rust"

    ```rust
    --8<-- "examples/19_get_wallet_tx_list.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/GetWalletTxList19.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/19_get_wallet_tx_list.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/19-get_wallet_tx_list.ts"
    ```

## 20. Send Compliment

=== "Rust"

    ```rust
    --8<-- "examples/20_send_compliment.rs"
    ```

=== "Java"

    ```java
    --8<-- "bindings/android/tests/src/main/java/com/etogruppe/examples/SendCompliment20.java"
    ```

=== "Swift"

    ```swift
    --8<-- "bindings/swift_new/examples/Sources/20_send_compliment.swift"
    ```

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/20-send_compliment.ts"
    ```

## 21. Initialize Wallet from Mnemonic

=== "Rust"

    ```rust
    --8<-- "examples/21_init_wallet_from_mnemonic.rs"
    ```

=== "Java"

    Not available.

=== "Swift"

    Not available.

=== "JS/TS"

    ```typescript
    --8<-- "bindings/wasm/examples/21-init_wallet_from_mnemonic.ts"
    ```
