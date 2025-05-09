use super::IotaAddress;

/// A [`core::result::Result`] with [`RebasedError`] as its error variant.
pub type Result<T> = core::result::Result<T, RebasedError>;

/// Wrapper for Iota Rebased Errors
#[derive(thiserror::Error, Debug)]
pub enum RebasedError {
    #[error("InvalidAppId")]
    InvalidAppId,
    #[error("InvalidIntentVersion")]
    InvalidIntentVersion,
    #[error("InvalidIntentScope")]
    InvalidIntentScope,
    #[error("InvalidIntent")]
    InvalidIntent,

    #[error("InvalidCryptoInput")]
    InvalidInput,

    #[error("InputLengthWrong: {0}")]
    InputLengthWrong(usize),

    #[error("InputTooShort: {0}")]
    InputTooShort(usize),

    #[error("InvalidSignature")]
    InvalidSignature,

    #[error("RpcError: {0}")]
    RpcError(#[from] jsonrpsee::core::client::Error),

    #[error("InvalidAddress")]
    InvalidAddress,

    #[error("InvalidDigestLength: Expected 32 bytes")]
    InvalidDigestLength,

    #[error("KeyNotFound for address {address}")]
    KeyNotFound { address: IotaAddress },

    #[error("KeyConversion: {0}")]
    KeyConversion(String),

    #[error("SizeOneVec: expected a vec of size 1")]
    SizeOneVecSize,

    #[error("InvalidIdentifier: `{0}`")]
    InvalidIdentifier(String),

    #[error("Mnemonic: {0}")]
    Mnemonic(#[from] bip39::ErrorKind),

    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("BuilderError: {0}")]
    BuilderError(#[from] super::BuilderError),

    #[error("Bcs: {0}")]
    Bcs(#[from] bcs::Error),

    #[error("Base64: {0}")]
    Base64(#[from] base64ct::Error),

    #[error("Hex: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Base58: {0}")]
    Base58(#[from] bs58::decode::Error),

    #[error("Failure deserializing object in the requested format: {:?}", error)]
    ObjectDeserialization { error: String },

    #[error("Failure serializing object in the requested format: {:?}", error)]
    ObjectSerialization { error: String },

    #[error("LayoutBuilder: {0}")]
    LayoutBuilderError(String),

    #[error("IotaEvent: {0}")]
    EventError(String),

    #[error("ParseIntError: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("ParsedError: {0}")]
    ParsedAddressError(String),

    #[error("ParserError: {0}")]
    ParserError(String),
}
