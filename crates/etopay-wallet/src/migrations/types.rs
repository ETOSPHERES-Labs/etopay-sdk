use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{WalletTxInfoV1, WalletTxInfoV2};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VersionedWalletTransaction {
    V1(WalletTxInfoV1),
    V2(WalletTxInfoV2),
}

impl VersionedWalletTransaction {
    pub fn date(&self) -> DateTime<Utc> {
        match self {
            VersionedWalletTransaction::V1(w) => w.date,
            VersionedWalletTransaction::V2(w) => w.date,
        }
    }
}

impl From<WalletTransaction> for VersionedWalletTransaction {
    fn from(value: WalletTransaction) -> Self {
        Self::V2(WalletTxInfoV2 {
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

// Points to the latest version
pub type WalletTransaction = WalletTxInfoV2;

impl From<VersionedWalletTransaction> for WalletTransaction {
    fn from(value: VersionedWalletTransaction) -> Self {
        match value {
            VersionedWalletTransaction::V1(v1) => WalletTxInfoV2 {
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
