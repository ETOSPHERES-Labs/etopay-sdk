use serde::{Deserialize, Serialize};

/// Struct for kyc status response
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct KycStatusResponse {
    /// Username
    pub username: String,
    /// Flag if user is verified
    pub is_verified: bool,
}
