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

    /// Error occurs when the user password is weak
    #[error("Weak password")]
    WeakPassword,

    /// Error raises if the pin is empty
    #[error("Pin should not be empty")]
    EmptyPin,

    /// Error occurs when the user pin is less than 6 digits
    #[error("Weak pin")]
    WeakPin,

    /// Error occurs when the user pin contains non numerical values
    #[error("Pin should only contain numbers")]
    NonNumericPin,

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
