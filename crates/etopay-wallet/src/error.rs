/// A [`core::result::Result`] with [`WalletError`] as its error variant.
pub type Result<T> = core::result::Result<T, WalletError>;

/// Wrapper for wallet errors
#[derive(thiserror::Error, Debug)]
pub enum WalletError {
    /// Error raises if the feature is not implemented
    #[error("Wallet feature is not implemented")]
    WalletFeatureNotImplemented,

    /// Error raises if the wallet address is empty
    #[error("Wallet address is empty")]
    EmptyWalletAddress,

    /// Error raises if something failed to parse
    #[error("ParseError: {0}")]
    Parse(String),

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

    /// Error occurred while handling bip32 compliant derivation paths
    #[error("Bip32: {0:?}")]
    Bip32(#[from] bip32::Error),

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
    IotaRebased(#[from] crate::rebased::RebasedError),

    /// Failed to wait for confirming the transaction status
    #[error("FailToConfirmTransactionStatus: Failed to confirm tx status for {0} within {1} seconds.")]
    FailToConfirmTransactionStatus(String, u64),
}

impl From<rust_decimal::Error> for WalletError {
    fn from(value: rust_decimal::Error) -> Self {
        Self::Decimal(value)
    }
}
