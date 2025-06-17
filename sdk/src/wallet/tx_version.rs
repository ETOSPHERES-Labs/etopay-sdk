use chrono::{DateTime, Utc};
use etopay_wallet::types::{CryptoAmount, WalletTransaction};
use serde::{Deserialize, Serialize};

use crate::types::WalletTxStatus;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Legacy format (version 1) of a wallet transaction.
///
/// This structure represents the original schema used for wallet transactions
/// before the introduction of `WalletTransaction` in version 2. It is retained
/// for backward compatibility and data migration purposes.
pub struct WalletTxInfoV1 {
    /// Timestamp of the transaction.
    pub date: DateTime<Utc>,
    /// Optional block number and block hash associated with the transaction.
    pub block_number_hash: Option<(u64, String)>,
    /// Unique identifier (hash) of the transaction.
    pub transaction_hash: String,
    /// Wallet address of the sender.
    pub sender: String,
    /// Wallet address of the receiver.
    pub receiver: String,
    /// Amount of cryptocurrency transferred.
    pub amount: CryptoAmount,
    /// Identifier for the network.
    pub network_key: String,
    /// Status of the transaction.
    pub status: WalletTxStatus,
    /// Optional link to a blockchain explorer showing the transaction details.
    pub explorer_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// A versioned representation of a wallet transaction.
pub enum VersionedWalletTransaction {
    /// Legacy transaction format (version 1).
    V1(WalletTxInfoV1),
    /// Current transaction format (version 2).
    V2(WalletTransaction),
}

impl VersionedWalletTransaction {
    /// Returns the timestamp of the transaction.
    pub fn date(&self) -> DateTime<Utc> {
        match self {
            VersionedWalletTransaction::V1(w) => w.date,
            VersionedWalletTransaction::V2(w) => w.date,
        }
    }

    /// Returns the transaction hash.
    pub fn transaction_hash(&self) -> &str {
        match self {
            VersionedWalletTransaction::V1(v1) => &v1.transaction_hash,
            VersionedWalletTransaction::V2(v2) => &v2.transaction_hash,
        }
    }

    /// Returns the network key associated with the transaction.
    pub fn network_key(&self) -> &str {
        match self {
            VersionedWalletTransaction::V1(v1) => &v1.network_key,
            VersionedWalletTransaction::V2(v2) => &v2.network_key,
        }
    }

    /// Returns the current status of the transaction.
    pub fn status(&self) -> WalletTxStatus {
        match self {
            VersionedWalletTransaction::V1(v1) => v1.status,
            VersionedWalletTransaction::V2(v2) => v2.status,
        }
    }
}

impl From<WalletTransaction> for VersionedWalletTransaction {
    fn from(value: WalletTransaction) -> Self {
        Self::V2(WalletTransaction {
            date: value.date,
            block_number_hash: value.block_number_hash,
            transaction_hash: value.transaction_hash,
            sender: value.sender,
            receiver: value.receiver,
            amount: value.amount,
            network_key: value.network_key,
            status: value.status,
            explorer_url: value.explorer_url,
            gas_fee: value.gas_fee,
            is_sender: value.is_sender,
        })
    }
}

impl From<VersionedWalletTransaction> for WalletTransaction {
    fn from(value: VersionedWalletTransaction) -> Self {
        match value {
            VersionedWalletTransaction::V1(v1) => WalletTransaction {
                date: v1.date,
                block_number_hash: v1.block_number_hash,
                transaction_hash: v1.transaction_hash,
                sender: v1.sender,
                receiver: v1.receiver,
                amount: v1.amount,
                network_key: v1.network_key,
                status: v1.status,
                explorer_url: v1.explorer_url,
                gas_fee: None,
                is_sender: false,
            },
            VersionedWalletTransaction::V2(v2) => v2,
        }
    }
}
