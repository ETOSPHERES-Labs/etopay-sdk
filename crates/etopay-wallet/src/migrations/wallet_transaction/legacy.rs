use crate::{
    MigratableWalletTx, MigrationStatus, WithMigrationStatus,
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

pub fn migrate_legacy_transactions_to_v1(txs: Vec<WalletTxInfo>) -> Vec<MigratableWalletTx> {
    txs.into_iter()
        .map(|tx| MigratableWalletTx::V1(WithMigrationStatus::new(tx.into(), MigrationStatus::Completed)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CryptoAmount, WalletTxInfo, WalletTxStatus};
    use chrono::Utc;

    #[test]
    fn test_migrate_legacy_to_v1() {
        // Given
        let legacy = WalletTxInfo {
            date: Utc::now().to_rfc3339(),
            transaction_hash: String::from("0xabc"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::from("network_key"),
            status: WalletTxStatus::Confirmed,
            block_number_hash: None,
            explorer_url: Some(String::from("https://explorer")),
        };

        // When
        let result = migrate_legacy_transactions_to_v1(vec![legacy.clone()]);

        // Then
        assert_eq!(result.len(), 1);
        match &result[0] {
            MigratableWalletTx::V1(wrapped) => {
                let v1 = &wrapped.data;
                assert_eq!(v1.transaction_hash, legacy.transaction_hash);
                assert_eq!(v1.sender, legacy.sender);
                assert_eq!(v1.receiver, legacy.receiver);
                assert_eq!(v1.amount, legacy.amount);
                assert_eq!(v1.network_key, legacy.network_key);
                assert_eq!(v1.status, legacy.status);
                assert_eq!(wrapped.migration_status, MigrationStatus::Completed);
            }
            _ => panic!("Expected V1 variant"),
        }
    }
}
