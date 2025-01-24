use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserDataRequest {
    /// Email address of the user
    pub mail: String,
    /// If the user accepts the terms
    pub terms_accepted: bool,
}

#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserDataResponse {
    /// Username
    pub username: String,
}
