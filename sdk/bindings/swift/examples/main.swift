let message = "swift bindings test"
let data = message.data(using: .utf8)!
let bytes = [Uint8](data)

let buf: [UInt8] = Array(message.utf8)
