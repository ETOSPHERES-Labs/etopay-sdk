import Foundation

// Extend the converted Rust strings into Swift strings (RustString) to print them as error strings
extension RustString: @unchecked Sendable {}
extension RustString: Error {}
