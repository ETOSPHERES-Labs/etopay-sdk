use serde::{Deserialize, Serialize};

/// Struct to upload the shares from SDK as Strings
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PutSharesRequest {
    pub backup_share: String,
    pub recovery_share: String,
}

/// Struct to download the shares in SDK as Strings
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetShareResponse {
    pub share: String,
}
