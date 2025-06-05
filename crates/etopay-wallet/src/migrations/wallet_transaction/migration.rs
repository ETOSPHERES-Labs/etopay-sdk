use serde::{Deserialize, Serialize};

use crate::{
    Migrate, MigrationStatus, WithMigrationStatus,
    types::{WalletTxInfoV1, WalletTxInfoV2},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "version")]
pub enum WalletTxInfoVersioned {
    V1(WithMigrationStatus<WalletTxInfoV1>),
    V2(WithMigrationStatus<WalletTxInfoV2>),
}

impl WalletTxInfoVersioned {
    pub fn into_latest(self) -> WithMigrationStatus<WalletTxInfoV2> {
        match self {
            WalletTxInfoVersioned::V1(v1) => v1.migrate(),
            WalletTxInfoVersioned::V2(v2) => v2,
        }
    }
}

impl Migrate for WithMigrationStatus<WalletTxInfoV1> {
    type Next = WithMigrationStatus<WalletTxInfoV2>;

    fn migrate(self) -> Self::Next {
        let v1 = self.data;

        WithMigrationStatus::new(
            WalletTxInfoV2 {
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
            },
            MigrationStatus::Pending,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{CryptoAmount, WalletTxStatus};

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_into_latest_from_v1() {
        // Given
        let date = Utc::now();
        let v1 = WalletTxInfoV1 {
            date,
            transaction_hash: "0x000".to_string(),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(3),
            network_key: "network".to_string(),
            status: WalletTxStatus::Confirmed,
            block_number_hash: None,
            explorer_url: None,
        };

        let wrapped = WithMigrationStatus::new(v1, MigrationStatus::Pending);
        let versioned = WalletTxInfoVersioned::V1(wrapped);

        // When
        let latest = versioned.into_latest();

        // Then
        assert_eq!(latest.data.transaction_hash, "0x000");
        assert_eq!(latest.data.gas_fee, None); // field introduced with v2
        // `Pending`` because we need to update gas_fee (manually)
        assert_eq!(latest.migration_status, MigrationStatus::Pending);
    }
}
