//! The Sdk struct is responsible for handling the Viviswap-related functionality and acts as a bridge between the Viviswap API and the application.
#[cfg(feature = "viviswap-swap")]
mod swap;

#[cfg(feature = "viviswap-kyc")]
mod kyc;

/// Viviswap related errors
#[derive(Debug, thiserror::Error)]
pub enum ViviswapError {
    /// Error raises if field for kyc verification is invalid
    #[error("Viviswap validation error. Invalid field: {0}. Reason: {1}")]
    Validation(String, String),

    /// Error raises viviswap state is invalid
    #[error("Viviswap invalid state error")]
    InvalidState,

    /// Error occurs if viviawap user has an existing state
    #[error("Viviswap user state existing")]
    UserStateExisting,

    /// Error raises if there is an error with viviswap api
    #[error("Viviswap api error: {0}")]
    Api(String),

    /// Error occurs is viviswap user is not found
    #[error("Missing Viviswap user")]
    MissingUser,

    /// Error occurs if a filed is missing
    #[error("Viviswap `{field}` is empty")]
    MissingField {
        /// the name of the field which is missing
        field: String,
    },

    /// Variant to hold a collection of errors
    #[error("Aggregate errors: {:?}", 0)]
    Aggregate(Vec<crate::Error>),
}
