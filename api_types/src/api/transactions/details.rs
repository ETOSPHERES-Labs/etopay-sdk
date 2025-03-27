use crate::api::{decimal::Decimal, networks::ApiNetwork};

use super::ApiTxStatus;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::IntoParams))]
pub struct TransactionDetailsQuery {
    /// Transaction index
    pub index: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetTransactionDetailsResponse {
    /// Address where to send the amount
    pub system_address: String,
    /// Total amount of crypto currency to send
    pub amount: Decimal,
    /// The Status of transaction
    pub status: ApiTxStatus,
    /// The network that the transaction is sent in
    pub network: ApiNetwork,
}

// Query parameters for querying details of multiple transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct TxsDetailsQuery {
    pub date: Option<String>,
    pub partner: Option<String>,
    pub is_sender: bool,
    pub start: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetTxsDetailsResponse {
    pub txs: Vec<ApiTransaction>,
}

/// Schema for saving and updating data in the database for each transaction
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiTransaction {
    /// The unique transaction index generated by the application for the current transaction
    pub index: String,
    /// The state of the transaction
    pub status: ApiTxStatus,
    /// The date and time of the creation of the transaction
    pub created_at: String,
    /// The date and time of update of the transaction
    pub updated_at: String,
    // Fee rate 19% -> 0.19
    pub fee_rate: Decimal,
    /// The details of the transfer to ETO
    pub incoming: ApiTransferDetails,
    /// The details of the transfer to receiver
    pub outgoing: ApiTransferDetails,
    /// The application metadata of the tx
    pub application_metadata: Option<ApiApplicationMetadata>,
}

/// The details of the transfer information for traceability
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiTransferDetails {
    /// The unique id assigned by the network/exchange for this transfer
    pub transaction_id: Option<String>,
    /// The block id assigned by the network/exchange for this transfer
    pub block_id: Option<String>,
    /// The username of the sender / receiver
    pub username: String,
    /// The address of the sender / receiver
    pub address: String,
    /// The network used to transfer amount between the sender and the receiver
    pub network: ApiNetwork,
    /// The amount to be transferred between the sender and the receiver
    pub amount: Decimal,
    /// The exchange rate from EUR to the decided currency.
    pub exchange_rate: Decimal,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ApiApplicationMetadata {
    pub product_hash: String,
    pub reason: String,
    pub purchase_model: String,
    pub app_data: String,
}
