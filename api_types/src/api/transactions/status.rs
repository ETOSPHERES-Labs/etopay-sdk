use serde::{Deserialize, Serialize};
use std::fmt;

/// The state of the transaction
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ApiTxStatus {
    Pending,
    WaitingForVerification(Vec<String>),
    Valid,
    Invalid(Vec<String>),
    ProcessingIncoming,
    ProcessingOutgoing,
    Completed,
    Failed,
}

impl fmt::Display for ApiTxStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema, utoipa::IntoParams))]
pub struct TransactionStatusQuery {
    /// Transaction index
    pub index: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetTransactionStatusRequest {
    pub index: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct GetTransactionStatusResponse {
    pub status: ApiTxStatus,
}
