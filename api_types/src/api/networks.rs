use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiNetworkType {
    Evm { node_urls: Vec<String>, chain_id: u64 },
    Stardust { node_urls: Vec<String> },
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiNetwork {
    pub id: String,
    pub name: String,
    pub currency: String,
    pub block_explorer_url: String,
    pub enabled: bool,
    pub network_identifier: Option<String>,
    pub network_type: ApiNetworkType,
}
