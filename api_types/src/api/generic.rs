use serde::{Deserialize, Serialize};

/// Supported crypto currencies
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiCryptoCurrency {
    /// Iota Crypto Currency
    Iota,
    /// Ethereum Crypto Currency
    Eth,
}

/// Struct for storing the commit hash from different services
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct HealthResponse {
    pub version: String,
}
