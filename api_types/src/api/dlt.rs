use super::networks::ApiNetwork;
use crate::api::decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Get/set system/user address query parameters
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct AddressQueryParameters {
    pub network_key: String,
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
pub struct SetPreferredNetworkRequest {
    /// The key to the preferred network, or None if it should be cleared.
    pub network_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetPreferredNetworkResponse {
    /// The input string representing the network key, or None if it is not set.
    pub network_key: Option<String>,
}

// Get networks from the backend
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiGetNetworksResponse {
    pub networks: Vec<ApiNetwork>,
}

// data objects

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Course {
    pub course: Decimal,
    pub date: String,
}

// requests

// get course

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseRequestQueries {
    pub network_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseResponse {
    pub course: Course,
}

// get course history

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct GetCourseHistoryRequestQueries {
    pub network_key: String,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetCourseHistoryResponse {
    pub courses: Vec<Course>,
}
