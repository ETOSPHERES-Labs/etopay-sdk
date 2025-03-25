use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiProtocol {
    Evm { chain_id: u64 },
    EvmERC20 { chain_id: u64, contract_address: String },
    Stardust {},
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Structure representing a cryptocurrency network.
pub struct ApiNetwork {
    /// Unique key for the network
    pub key: String,
    /// Display name of the network
    pub display_name: String,
    /// Symbol of the network's currency
    pub display_symbol: String,
    /// Coin type, as defined by SLIP-0044 standard
    pub coin_type: u32,
    /// List of node URLs for the network
    pub node_urls: Vec<String>,
    /// Number of decimal places for the network's currency unit
    pub decimals: u32,
    /// Whether the network supports purchase transactions
    pub can_do_purchases: bool,
    /// Protocol used by the network
    pub protocol: ApiProtocol,
    /// URL of the network's block explorer
    pub block_explorer_url: String,
}
