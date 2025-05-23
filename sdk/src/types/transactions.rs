use api_types::api::{
    networks::ApiNetwork,
    transactions::{ApiApplicationMetadata, ApiTxStatus},
};
use etopay_wallet::types::CryptoAmount;
use serde::Serialize;

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
