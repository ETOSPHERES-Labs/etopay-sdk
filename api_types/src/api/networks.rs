use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiProtocol {
    /// Represents an EVM-based network (e.g., Ethereum)
    Evm {
        /// chain_id
        chain_id: u64,
    },
    /// Represents and EVM based ERC20 Smart Contract token
    EvmERC20 {
        /// chain_id
        chain_id: u64,
        ///contract address
        contract_address: String,
    },
    /// Represents a Stardust network
    Stardust {},
    IotaRebased {},
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
/// Represents a network supported by the wallet
pub struct ApiNetwork {
    /// Unique key for the network
    pub key: String,
    /// If this network is a test network
    pub is_testnet: bool,
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
