// The Swift Programming Language
// https://docs.swift.org/swift-book

import ETOPaySdk
import Foundation
import utils

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

Task {
    do {
        // Get environment variables from the Utils module
        let env = getEnvironment()

        // Initialize SDK
        let sdk = try await initSdk(username: env.username, password: env.password)

        // Create new user
        try await sdk.createNewUser(env.username)
        print("created new user: \(env.username)")
        try await sdk.initUser(env.username)
        print("initialized new user: \(env.username)")

        // Migrate wallet
        try await sdk.setWalletPassword(env.pin, env.wallet_password)
        let _ = try await sdk.createWalletFromMnemonic(env.pin, env.mnemonic)
        print("migrated wallet from mnemonic")

        // Fetch networks from backend
        let _ = try await sdk.getNetworks()
        try await sdk.setNetwork("iota_rebased_testnet")
        print("retrieved available networks and set the network for the wallet")

        // Generate address
        let address1 = try await sdk.generateNewAddress(env.pin)
        print("First Address: \(address1.toString())")

        try await sdk.setWalletAccount(0, 1)

        let address2 = try await sdk.generateNewAddress(env.pin)
        print("Second Address: \(address2.toString())")

    } catch let error as RustString {
        fatalError("Generate new iota address example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
