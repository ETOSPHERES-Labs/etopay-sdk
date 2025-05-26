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
        let address = try await sdk.generateNewAddress(env.pin)
        print("generated new receiver address: \(address.toString())")

        // Get wallet balance
        let balance = try await sdk.getWalletBalance(env.pin)
        print("balance: \(balance)")

        let message = "swift bindings test"

        // convert to a RustVec by copying over all values
        let rustVec = RustVec<UInt8>.init()
        for byte in message.utf8 {
            rustVec.push(value: byte)
        }

        // Estimate gas
        let estimate = try await sdk.estimateGas(env.pin, address.toString(), 1, rustVec)
        print("estimate: \(estimate.gas_limit.toString())")

        // Send amount
        let tx_id = try await sdk.sendAmount(env.pin, address.toString(), 1, rustVec)
        print("sent amount of 1 on transaction \(tx_id.toString())")

        // Get new balance
        let new_balance = try await sdk.getWalletBalance(env.pin)
        print("new balance: \(new_balance)")

        // Get the details (wrap env.pin in RustString to make sure parameters have the same type (they share same generic type))
        let details = try await sdk.getWalletTransaction(RustString(env.pin), tx_id)
        print("tx details status: \(details.status())")
        print("tx details amount: \(details.amount())")
        print("tx details receiver: \(details.receiver().toString())")
        print("tx details block_number: \(details.block_number())")


    } catch let error as RustString {
        fatalError("Send amount example failed: \(error.toString())")
    } catch {
        fatalError("Unexpected error occurred: \(error)")
    }

    group.leave()
}

group.wait()
