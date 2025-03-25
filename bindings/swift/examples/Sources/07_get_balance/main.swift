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
        try await sdk.setWalletPassword(env.pin, env.password)
        let _ = try await sdk.createWalletFromMnemonic(env.pin, env.mnemonic)
        print("migrated wallet from mnemonic")

        // Fetch networks from backend
        let networks = try await sdk.getNetworks()
        try await sdk.setNetwork(networks[0].key())
        print("retrieved available networks and set the network for the wallet")

        // Generate address
        let address = try await sdk.generateNewAddress(env.pin)
        print("generated new receiver address: \(address.toString())")

        // Get wallet balance
        let balance = try await sdk.getWalletBalance(env.pin)
        print("balance: \(balance)")

    } catch let error as RustString {
        fatalError("Get wallat balance failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
