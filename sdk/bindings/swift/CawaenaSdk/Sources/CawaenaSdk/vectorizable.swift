import CawaenaSdkBin
import SystemConfiguration

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
extension RustString: @unchecked Sendable {}
extension RustString: Error {}

// Ensure that TxInfo conform to the Vectorizable protocol so we can brige Vec<TxInfo>
extension TxInfo: Vectorizable {
    public typealias SelfRef = TxInfo
    public typealias SelfRefMut = TxInfo

    public static func vecOfSelfNew() -> UnsafeMutableRawPointer {
        return UnsafeMutableRawPointer.allocate(
            byteCount: MemoryLayout<TxInfo>.stride, alignment: MemoryLayout<TxInfo>.alignment)
    }

    public static func vecOfSelfFree(vecPtr: UnsafeMutableRawPointer) {
        vecPtr.deallocate()
    }

    public static func vecOfSelfPush(vecPtr: UnsafeMutableRawPointer, value: TxInfo) {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        valuePtr.initialize(to: value)
    }

    public static func vecOfSelfPop(vecPtr: UnsafeMutableRawPointer) -> TxInfo? {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        defer { valuePtr.deinitialize(count: 1) }
        return valuePtr.pointee
    }

    public static func vecOfSelfGet(vecPtr: UnsafeMutableRawPointer, index: UInt) -> TxInfo.SelfRef?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfGetMut(vecPtr: UnsafeMutableRawPointer, index: UInt) -> TxInfo
        .SelfRefMut?
    {
        guard index == 0 else { return nil }
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return valuePtr.pointee
    }

    public static func vecOfSelfAsPtr(vecPtr: UnsafeMutableRawPointer) -> UnsafePointer<
        TxInfo.SelfRef
    > {
        let valuePtr = vecPtr.bindMemory(to: TxInfo.self, capacity: 1)
        return UnsafePointer(valuePtr)
    }

    public static func vecOfSelfLen(vecPtr: UnsafeMutableRawPointer) -> UInt {
        return 1
    }
}

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
