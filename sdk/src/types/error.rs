/// A [`core::result::Result`] with [`TypeError`] as its error variant.
pub type Result<T> = core::result::Result<T, TypeError>;

/// Errors related to sdk types
#[derive(thiserror::Error, Debug)]
pub enum TypeError {
    /// Error raises if the currency used is invalid / not supported
    #[error("InvalidCurrency: {0}")]
    InvalidCurrency(String),

    /// Error raises if the password is empty
    #[error("Password should not be empty")]
    EmptyPassword,

    /// Error raises if the pin is empty
    #[error("Pin should not be empty")]
    EmptyPin,

    /// Error raises if the access token is empty
    #[error("Access token should not be empty")]
    EmptyAccessToken,

    /// Error raises if the password fails to be encrypted
    #[error("Unable to encrypt password.")]
    PasswordEncryption,

    /// Error raises if the pin or password is incorrect
    #[error("Pin or password is invalid.")]
    InvalidPinOrPassword,
}
