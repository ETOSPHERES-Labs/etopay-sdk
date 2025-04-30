use super::{kdbx::KdbxStorageError, rebased, share::ShareError};
use crate::{backend::error::ApiError, types::error::TypeError, user::error::UserKvStorageError};
use iota_sdk::types::block;
use serde::Serialize;

/// A [`core::result::Result`] with [`WalletError`] as its error variant.
pub type Result<T> = core::result::Result<T, WalletError>;

#[derive(Debug, Serialize, PartialEq, Clone, Copy)]
/// Kind of error contained in [`WalletError`]
pub enum ErrorKind {
    /// You need to set the password before you can initialize the wallet.
    MissingPassword,
    /// You need to set / upload the recovery share before you can initialize the wallet.
    SetRecoveryShare,
    /// You need to use the mnemonic or create a wallet before you can use the wallet.
    UseMnemonic,
}

/// Wrapper for wallet errors
#[derive(thiserror::Error, Debug)]
pub enum WalletError {
    /// Iota client error
    #[error("IotaClient error: {0}")]
    IotaClient(#[from] iota_sdk::client::Error),

    /// Error occurs if password is missing
    #[error("Password is missing")]
    MissingPassword,

    /// Wrong pin or password
    #[error("Pin or password incorrect.")]
    WrongPinOrPassword,

    /// Error occurs if the wallet is not initialized
    #[error("Wallet init error: {0:?}")]
    WalletNotInitialized(ErrorKind),

    /// Error raises if the feature is not implemented
    #[error("Wallet feature is not implemented")]
    WalletFeatureNotImplemented,

    /// Error raises if authentication token is outdated or invalid
    #[error("Unauthorized: Missing Access Token")]
    MissingAccessToken,

    /// Error raises if the wallet address is empty
    #[error("Wallet address is empty")]
    EmptyWalletAddress,

    /// Error raises if something failed to parse
    #[error("ParseError: {0}")]
    Parse(String),

    /// Alloy RPC error
    #[error("Alloy RPC error: {0}")]
    Rpc(String),

    /// Error occurs is the transaction amount is invalid
    #[error("InvalidTransactionAmount: {0}")]
    InvalidTransactionAmount(String),

    /// Error occurs is the transaction is invalid
    #[error("InvalidTransaction: {0}")]
    InvalidTransaction(String),

    /// Insufficient balance on wallet
    #[error("InsufficientBalanceError: {0}")]
    InsufficientBalance(String),

    /// Error caused by conversions to/from Decimal and f64
    #[error("Decimal error: {0}")]
    Decimal(rust_decimal::Error),

    /// Block error
    #[error("Block error: {0}")]
    Block(#[from] block::Error),

    /// Iota wallet error
    #[error("IotaWallet error: {0}")]
    IotaWallet(#[from] iota_sdk::wallet::Error),

    /// Errors related to the kdbx storage
    #[error("KdbxStorage error: {0}")]
    KdbxStorage(#[from] KdbxStorageError),

    /// Error occurs in sdk types
    #[error("Type errors: {0}")]
    Type(#[from] TypeError),

    /// User repository error
    #[error("User repository error: {0}")]
    UserRepository(#[from] UserKvStorageError),

    /// Error occurred while creating or reconstructing shares
    #[error("Share error: {0}")]
    Share(#[from] ShareError),

    /// Error occurred while handling bip39 compliant mnemonics
    #[error("Bip39 error: {0:?}")]
    Bip39(iota_sdk::crypto::keys::bip39::Error),

    /// Error occurred while handling bip32 compliant derivation paths
    #[error("Bip32: {0:?}")]
    Bip32(#[from] bip32::Error),

    /// Error occurs in sdk backend (api)
    #[error("BackendApi errors: {0}")]
    BackendApi(#[from] ApiError),

    /// Error creating a LocalSigner from the provided mnemonic
    #[error("LocalSignerError: {0}")]
    LocalSignerError(#[from] alloy::signers::local::LocalSignerError),

    /// Error waiting for transaction to be included
    #[error("PendingTransactionError: {0}")]
    PendingTransactionError(#[from] alloy::providers::PendingTransactionError),

    /// Could not convert hex to address
    #[error("Invalid hex value: {0}")]
    FromHexError(#[from] alloy_primitives::hex::FromHexError),

    /// Alloy transport error
    #[error("Alloy transport RPC error: {0}")]
    AlloyTransportRpcError(#[from] alloy_json_rpc::RpcError<alloy_transport::TransportErrorKind>),

    /// Error raises if transaction does not exist
    #[error("TransactionNotFound")]
    TransactionNotFound,

    /// Error raises if value cannot be converted
    #[error("Unable to convert: {0}")]
    ConversionError(String),

    /// Error for calling a Smart Contract
    #[error("Contract error: {0}")]
    Contract(#[from] alloy::contract::Error),

    /// Error for decoding a Smart Contract call
    #[error("SolidityError error: {0}")]
    SolidityError(#[from] alloy::sol_types::Error),

    /// Iota Rebased Error
    #[error("IotaRebased: {0}")]
    IotaRebased(#[from] rebased::RebasedError),

    /// Failed to wait for confirming the transaction status
    #[error("FailToConfirmTransactionStatus: Failed to confirm tx status for {0} within {1} seconds.")]
    FailToConfirmTransactionStatus(String, u64),
}

impl From<iota_sdk::crypto::keys::bip39::Error> for WalletError {
    fn from(value: iota_sdk::crypto::keys::bip39::Error) -> Self {
        Self::Bip39(value)
    }
}

impl From<rust_decimal::Error> for WalletError {
    fn from(value: rust_decimal::Error) -> Self {
        Self::Decimal(value)
    }
}
