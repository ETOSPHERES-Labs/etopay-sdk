use serde::{Deserialize, Serialize};

use crate::types::{WalletTxInfo, WalletTxInfoV1, WalletTxInfoV2, parse_date_or_default};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    // indicates that after migrating to the new version, some fields need to be filled in manually
    Pending,
    // indicates that the object is fully populated and the migration process is complete
    Completed,
}

impl Default for MigrationStatus {
    fn default() -> Self {
        MigrationStatus::Completed
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WithMigrationStatus<T> {
    #[serde(skip)]
    pub migration_status: MigrationStatus,

    #[serde(flatten)]
    pub data: T,
}

impl<T> WithMigrationStatus<T> {
    pub fn new(data: T, status: MigrationStatus) -> Self {
        Self {
            data,
            migration_status: status,
        }
    }

    pub fn mark_completed(mut self) -> Self {
        self.migration_status = MigrationStatus::Completed;
        self
    }

    pub fn inner(&self) -> &T {
        &self.data
    }

    pub fn into_inner(self) -> T {
        self.data
    }
}

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

// migrate legacy struct
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

pub fn migrate_legacy_transactions_to_v1(txs: Vec<WalletTxInfo>) -> Vec<WalletTxInfoVersioned> {
    txs.into_iter()
        .map(|tx| WalletTxInfoVersioned::V1(WithMigrationStatus::new(tx.into(), MigrationStatus::Completed)))
        .collect()
}

pub trait Migrate {
    type Next;
    fn migrate(self) -> Self::Next;
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

// type WalletTxV1WithStatus = WithMigrationStatus<WalletTxInfoV1>;
// type WalletTxV2WithStatus = WithMigrationStatus<WalletTxInfoV2>;

// impl Migrate for WalletTxV1WithStatus {
//     type Next = WalletTxV2WithStatus;

//     fn migrate(self) -> Self::Next {
//         let v1 = self.data;

//         WithMigrationStatus {
//             migration_status: MigrationStatus::Pending,
//             data: WalletTxInfoV2 {
//                 date: v1.date,
//                 block_number_hash: v1.block_number_hash,
//                 transaction_hash: v1.transaction_hash,
//                 sender: v1.sender,
//                 receiver: v1.receiver,
//                 amount: v1.amount,
//                 network_key: v1.network_key,
//                 status: v1.status,
//                 explorer_url: v1.explorer_url,
//                 gas_fee: None,
//             },
//         }
//     }
// }

// impl<T> WithMigrationStatus<T> {
//     pub fn mark_completed(mut self) -> Self {
//         self.migration_status = MigrationStatus::Completed;
//         self
//     }

//     pub fn inner(&self) -> &T {
//         &self.data
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(tag = "version")]
// pub enum WalletTxInfoVersioned {
//     V1(WalletTxInfoV1),
//     V2(WalletTxInfoV2),
// }

// impl WalletTxInfoVersioned {
//     pub fn into_latest(self) -> WalletTxInfoV2 {
//         match self {
//             WalletTxInfoVersioned::V1(v1) => v1.migrate(),
//             WalletTxInfoVersioned::V2(v2) => v2,
//         }
//     }
// }

// impl From<WalletTxInfo> for WalletTxInfoV1 {
//     fn from(legacy: WalletTxInfo) -> Self {
//         WalletTxInfoV1 {
//             date: parse_date_or_default(&legacy.date),
//             block_number_hash: legacy.block_number_hash,
//             transaction_hash: legacy.transaction_hash,
//             sender: legacy.sender,
//             receiver: legacy.receiver,
//             amount: legacy.amount,
//             network_key: legacy.network_key,
//             status: legacy.status,
//             explorer_url: legacy.explorer_url,
//             migration_status: MigrationStatus::Completed,
//         }
//     }
// }

// pub fn migrate_legacy_transactions_to_v1(legacy_list: Vec<WalletTxInfo>) -> Vec<WalletTxInfoVersioned> {
//     legacy_list
//         .into_iter()
//         .map(|tx| WalletTxInfoVersioned::V1(tx.into()))
//         .collect()
// }

// pub trait Migrate {
//     type Next;

//     fn migrate(self) -> Self::Next;
// }

// impl Migrate for WalletTxInfoV1 {
//     type Next = WalletTxInfoV2;
//     fn migrate(self) -> WalletTxInfoV2 {
//         WalletTxInfoV2 {
//             date: self.date,
//             block_number_hash: self.block_number_hash,
//             transaction_hash: self.transaction_hash,
//             sender: self.sender,
//             receiver: self.receiver,
//             amount: self.amount,
//             network_key: self.network_key,
//             status: self.status,
//             explorer_url: self.explorer_url,
//             gas_fee: None,
//             migration_status: MigrationStatus::Pending,
//         }
//     }
// }

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
            migration_status: MigrationStatus::Completed,
        };

        let expected = WalletTxInfoV2 {
            date,
            transaction_hash: "0x000".to_string(),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(3),
            network_key: "network".to_string(),
            status: WalletTxStatus::Confirmed,
            block_number_hash: None,
            explorer_url: None,
            gas_fee: None,
            migration_status: MigrationStatus::Pending,
        };

        // When
        let versioned = WalletTxInfoVersioned::V1(v1);
        let latest = versioned.into_latest();

        // Then
        assert_eq!(expected, latest);
    }
}
