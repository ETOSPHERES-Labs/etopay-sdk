use api_types::api::{
    networks::ApiNetwork,
    transactions::{ApiApplicationMetadata, ApiTxStatus},
};
use serde::{Deserialize, Serialize};

use super::currencies::CryptoAmount;

/// Transaction list
#[derive(Debug, Serialize)]
pub struct TxList {
    /// List of transaction info
    pub txs: Vec<TxInfo>,
}

/// Transaction info
#[derive(Debug, Serialize, Clone)]
pub struct TxInfo {
    /// Tx creation date, if available
    pub date: Option<String>,
    /// sender of the transaction
    pub sender: String,
    /// receiver of the transaction
    pub receiver: String,
    /// etopay reference id for the transaction
    pub reference_id: String,
    /// Application specific metadata attached to the tx
    pub application_metadata: Option<ApiApplicationMetadata>,
    /// Amount of transfer
    pub amount: f64,
    /// Currency of transfer
    pub currency: String,
    /// Status of the transfer
    pub status: ApiTxStatus,
    /// The transaction hash on the network
    pub transaction_hash: Option<String>,
    /// Exchange rate
    pub course: f64,
}
/// wallet transaction info
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WalletTxInfo {
    /// Tx creation date, if available
    pub date: String,
    /// Contains block id
    pub block_id: Option<String>,
    /// transaction id for particular transaction
    pub transaction_id: String,
    /// Describes type of transaction
    pub incoming: bool,
    /// The receiver of the transaction
    pub receiver: String,
    /// Amount of transfer
    pub amount: f64,
    /// Unique key representing a network
    pub network_key: String,
    /// Status of the transfer
    pub status: String,
    /// Url of network IOTA/ETH
    pub explorer_url: Option<String>, // ok
                                      // change based on the network either eth or iota
                                      // base explorer url for IOTA = https://explorer.iota.org/mainnet/block/[block_id]
                                      // base explorer url for EVM = [node url]
}

/// List of wallet transactions
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct WalletTxInfoList {
    /// Transactions that happens
    pub transactions: Vec<WalletTxInfo>,
}

/// Purchase details
#[derive(Clone)]
pub struct PurchaseDetails {
    /// The sender address where the fees goes to.
    pub system_address: String,
    /// The amount to be paid.
    pub amount: CryptoAmount,
    /// The status of transaction
    pub status: ApiTxStatus,
    /// The network that the transaction is sent in
    pub network: ApiNetwork,
}

/// Gas estimation (EIP-1559)
#[derive(Debug, PartialEq)]
pub struct GasCostEstimation {
    /// The maximum fee the sender is willing to pay per unit of gas.
    pub max_fee_per_gas: u128,
    /// The maximum tip the sender is willing to pay to miners (in EIP-1559).
    pub max_priority_fee_per_gas: u128,
    /// The maximum amount of gas that the transaction can consume.
    pub gas_limit: u64,
}

/// Possible InclusionStates for transactions
/// TODO: refine this (just copy from iota-sdk for now)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum InclusionState {
    Pending,
    Confirmed,
    Conflicting,
}
