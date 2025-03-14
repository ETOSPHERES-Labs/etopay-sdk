// we use this file as a `test` to verify that we have a connection established between Rust and Swift
// we can call our Rust bindings from this Swift file and test if we get the expected response
import Foundation

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
extension RustString: @unchecked Sendable {}
extension RustString: Error {}

// Ensure that WalletTxInfo conform to the Vectorizable protocol so we can brige Vec<WalletTxInfo>
extension WalletTxInfo: Vectorizable {
    public typealias SelfRef = WalletTxInfo
    public typealias SelfRefMut = WalletTxInfo

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<WalletTxInfo>.stride,
            alignment: MemoryLayout<WalletTxInfo>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: WalletTxInfo) {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> WalletTxInfo? {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> WalletTxInfo
        .SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> WalletTxInfo
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        WalletTxInfo.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: WalletTxInfo.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

// We use a DispatchGroup to make the program wait until async functions finish before exiting.
// Not needed in long-running applications.
let group = DispatchGroup()
group.enter()

print("We're in Swift about to call our async Rust functions.")
Task {
    do {
        let sdk = CawaenaSdk()

        try await sdk.setConfig(
            """
            {
                "backend_url": "http://test.url.com/api",
                "storage_path": ".",
                "log_level": "info",
                "auth_provider": "standalone"
            }
            """)
        print("sdk configured")

    } catch let error as RustString {  // catch string errors that we return from Rust
        print(error.toString())

    } catch {  // catch other unexpected error types
        print("unexpected error: \(error)")
    }

    group.leave()
}

group.wait()
