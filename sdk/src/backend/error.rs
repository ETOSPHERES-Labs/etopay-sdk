use reqwest::StatusCode;

/// A [`core::result::Result`] with [`ApiError`] as its error variant.
pub type Result<T> = core::result::Result<T, ApiError>;

/// Errors related to sdk backend
#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    /// API request error
    #[error("API request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Error raises if authentication token is outdated or invalid
    #[error("Unauthorized: Missing Access Token")]
    MissingAccessToken,

    /// Error raises if there are parsing errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// Unexpected response error
    #[error("Unexpected response: code: {code}, body: {body}")]
    UnexpectedResponse { code: StatusCode, body: String },

    /// Error raises if something is wrong with the shares (e.g. not encrypted)
    #[error("Share error: {0}")]
    Share(String),
}
