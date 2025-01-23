use super::generic::ApiCryptoCurrency;
use serde::{Deserialize, Serialize};

/// Represents the request parameters needed to collect user data from various services.
///
/// * `account-service` - Parameters: username. Data obtained: customer data.
/// * `dlt-service` - Parameters: username, method. Data obtained: user address.
/// * `viviswap-service` - Parameters: username, start, limit. Data obtained: orders.
/// * `postident-service` - Parameters: username. Data obtained: case details.
/// * `kyc-service` - Parameters: username. Data obtained: verified status.
/// * `transactions-service` - Parameters: username, date, option, partner, is_sender, start, limit. Data obtained: transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetUserDataQuery {
    pub date: Option<String>,
    pub option: Option<String>,
    pub partner: Option<String>,
    pub is_sender: bool,
    pub start: u32,
    pub limit: u32,
    pub payment_method_currency: ApiCryptoCurrency,
}

/// Struct to upload the shares from SDK as Strings
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PutShareRequest {
    pub share: String,
}

/// Struct to download the shares in SDK as Strings
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetShareResponse {
    pub share: String,
}
