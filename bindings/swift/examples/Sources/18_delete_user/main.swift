// The Swift Programming Language
// https://docs.swift.org/swift-book

import Foundation
import ETOPaySdk
import utils

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

Task {
    do {
        print("start")

        // Get environment variables from the Utils module
        let env = getEnvironment()
        print("env")
        print("env: \(env)")

        let username_archive = ProcessInfo.processInfo.environment["ARCHIEVEME"]!;
        print("username_archive")
        print("username_archive: \(username_archive)")

        // Initialize SDK
        let sdk = try await initSdk(username: username_archive, password: env.password)
        print("sdk initialized")
        
        // Create new user
        try await sdk.createNewUser(username_archive)
        print("created new user: \(username_archive)")
        try await sdk.initUser(username_archive)
        print("initialized new user: \(username_archive)")
        
        // Create new wallet
        try await sdk.setWalletPassword(env.pin, env.wallet_password)
        let _ = try await sdk.createNewWallet(env.pin)
        print("created new wallet")
        
        print("deleting user and wallet")
        try await sdk.deleteUser(env.pin)
        
        // check verification after deletion. Should be false
        let verified = try await sdk.isKycVerified(username_archive)
        print("is verified: \(verified)")

    } catch let error as RustString  {
        fatalError("Delete user example failed: \(error.toString())")
    }  catch {
        fatalError("Unexpected error occurred: \(error)")
    }
    
    group.leave()
}

group.wait()
