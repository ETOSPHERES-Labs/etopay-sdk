use super::generic::ApiCryptoCurrency;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Get/set system/user address query parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct AddressQueryParameters {
    pub currency: ApiCryptoCurrency,
}

/// Get User address request
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetUserAddressRequest {
    /// User address
    pub address: String,
}

/// Get user address response
#[derive(Default, Serialize, Deserialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetUserAddressResponse {
    /// User address
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct SetPreferredCurrencyRequest {
    /// The currency to set as the users preferred currency, or None if it should be cleared.
    pub currency: Option<ApiCryptoCurrency>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetPreferredCurrencyResponse {
    /// The currency set as the users preferred currency, or None if it is not set.
    pub currency: Option<ApiCryptoCurrency>,
}

// Struct to get node urls from backend
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiGetNodeUrlsResponse {
    pub node_urls: HashMap<String, Vec<String>>,
}
