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
    InvalidCryptoInput,

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

    #[error("FastCrypto: {0}")]
    FastCrypto(#[from] fastcrypto::error::FastCryptoError),

    #[error("Mnemonic: {0}")]
    Mnemonic(#[from] bip39::ErrorKind),

    #[error("FromUtf8Error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),

    #[error("BuilderError: {0}")]
    BuilderError(#[from] super::BuilderError),

    #[error("Bcs: {0}")]
    Bcs(#[from] bcs::Error),
}
