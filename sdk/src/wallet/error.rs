use super::{kdbx::KdbxStorageError, share::ShareError};
use crate::{backend::error::ApiError, types::error::TypeError, user::error::UserKvStorageError};
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

    /// Error raises if authentication token is outdated or invalid
    #[error("Unauthorized: Missing Access Token")]
    MissingAccessToken,

    /// Error occurs is the transaction is invalid
    #[error("InvalidTransaction: {0}")]
    InvalidTransaction(String),

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

    /// Error occurs in sdk backend (api)
    #[error("BackendApi errors: {0}")]
    BackendApi(#[from] ApiError),

    /// Error from the wallet impl
    #[error("WalletImplError: {0}")]
    WalletImplError(#[from] etopay_wallet::WalletError),
}

impl From<iota_sdk::crypto::keys::bip39::Error> for WalletError {
    fn from(value: iota_sdk::crypto::keys::bip39::Error) -> Self {
        Self::Bip39(value)
    }
}
