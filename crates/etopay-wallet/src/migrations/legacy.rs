use crate::{
    VersionedWalletTransaction,
    types::{WalletTxInfo, WalletTxInfoV1, parse_date_or_default},
};

impl From<WalletTxInfo> for WalletTxInfoV1 {
    fn from(legacy: WalletTxInfo) -> Self {
        WalletTxInfoV1 {
            date: parse_date_or_default(&legacy.date),
            block_number_hash: legacy.block_number_hash,
            transaction_hash: legacy.transaction_hash,
            sender: legacy.sender,
            receiver: legacy.receiver,
            amount: legacy.amount,
            network_key: legacy.network_key,
            status: legacy.status,
            explorer_url: legacy.explorer_url,
        }
    }
}

pub fn migrate_legacy_transactions_to_v1(txs: Vec<WalletTxInfo>) -> Vec<VersionedWalletTransaction> {
    txs.into_iter()
        .map(|tx| VersionedWalletTransaction::V1(tx.into()))
        .collect()
}
