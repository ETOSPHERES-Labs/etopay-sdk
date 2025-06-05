use crate::WalletTxInfoVersioned;

// pub fn sort_by_date(transactions: &mut [WalletTxInfoVersioned]) {
//     transactions.sort_by(|a, b| {
//         let a_date = match a {
//             WalletTxInfoVersioned::V1(wallet_tx_info_v1) => wallet_tx_info_v1.date,
//             WalletTxInfoVersioned::V2(wallet_tx_info_v2) => wallet_tx_info_v2.date,
//         };

//         let b_date = match b {
//             WalletTxInfoVersioned::V1(wallet_tx_info_v1) => wallet_tx_info_v1.date,
//             WalletTxInfoVersioned::V2(wallet_tx_info_v2) => wallet_tx_info_v2.date,
//         };

//         b_date.cmp(&a_date)
//     })
// }

pub fn sort_by_date(transactions: &mut [WalletTxInfoVersioned]) {
    transactions.sort_by(|a, b| {
        let a_date = match a {
            WalletTxInfoVersioned::V1(w) => w.data.date,
            WalletTxInfoVersioned::V2(w) => w.data.date,
        };

        let b_date = match b {
            WalletTxInfoVersioned::V1(w) => w.data.date,
            WalletTxInfoVersioned::V2(w) => w.data.date,
        };

        b_date.cmp(&a_date)
    });
}

// #[cfg(test)]
// mod tests {
//     use chrono::{DateTime, TimeZone, Utc};

//     use crate::types::{CryptoAmount, WalletTxInfoV1, WalletTxInfoV2};

//     use super::sort_by_date;

//     fn mock_transaction_v2(date: DateTime<Utc>) -> WalletTxInfoVersioned {
//         WalletTxInfoVersioned::V2(WalletTxInfoV2 {
//             date,
//             block_number_hash: None,
//             transaction_hash: String::from("tx_hash"),
//             sender: "Satoshi".to_string(),
//             receiver: "Bob".to_string(),
//             amount: CryptoAmount::from(1),
//             network_key: String::from("network"),
//             status: crate::types::WalletTxStatus::Pending,
//             explorer_url: None,
//             gas_fee: None,
//         })
//     }

//     fn mock_transaction_v1(date: DateTime<Utc>) -> WalletTxInfoVersioned {
//         WalletTxInfoVersioned::V1(WalletTxInfoV1 {
//             date,
//             block_number_hash: None,
//             transaction_hash: String::from("tx_hash"),
//             sender: "Satoshi".to_string(),
//             receiver: "Bob".to_string(),
//             amount: CryptoAmount::from(1),
//             network_key: String::from("network"),
//             status: crate::types::WalletTxStatus::Pending,
//             explorer_url: None,
//         })
//     }

//     #[test]
//     fn test_should_sort_by_date() {
//         // Given
//         let mut transactions = vec![
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 15).unwrap()),
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 14).unwrap()),
//             mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 13).unwrap()),
//             mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 16).unwrap()),
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 12).unwrap()),
//         ];

//         let expected = vec![
//             mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 16).unwrap()),
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 15).unwrap()),
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 14).unwrap()),
//             mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 13).unwrap()),
//             mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 12).unwrap()),
//         ];

//         // When
//         sort_by_date(&mut transactions);

//         // Then
//         assert_eq!(expected, transactions);
//     }
// }

#[cfg(test)]
mod tests {
    use super::sort_by_date;
    use crate::{
        MigrationStatus, WalletTxInfoVersioned, WithMigrationStatus,
        types::{CryptoAmount, WalletTxInfoV1, WalletTxInfoV2, WalletTxStatus},
    };
    use chrono::{DateTime, TimeZone, Utc};

    fn mock_transaction_v2(date: DateTime<Utc>) -> WalletTxInfoVersioned {
        let v2 = WalletTxInfoV2 {
            date,
            block_number_hash: None,
            transaction_hash: String::from("tx_hash"),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(1),
            network_key: String::from("network"),
            status: WalletTxStatus::Pending,
            explorer_url: None,
            gas_fee: None,
        };

        WalletTxInfoVersioned::V2(WithMigrationStatus::new(v2, MigrationStatus::Pending))
    }

    fn mock_transaction_v1(date: DateTime<Utc>) -> WalletTxInfoVersioned {
        let v1 = WalletTxInfoV1 {
            date,
            block_number_hash: None,
            transaction_hash: String::from("tx_hash"),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(1),
            network_key: String::from("network"),
            status: WalletTxStatus::Pending,
            explorer_url: None,
        };

        WalletTxInfoVersioned::V1(WithMigrationStatus::new(v1, MigrationStatus::Completed))
    }

    #[test]
    fn test_should_sort_by_date() {
        // Given
        let mut transactions = vec![
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 15).unwrap()),
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 14).unwrap()),
            mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 13).unwrap()),
            mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 16).unwrap()),
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 12).unwrap()),
        ];

        let expected = vec![
            mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 16).unwrap()),
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 15).unwrap()),
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 14).unwrap()),
            mock_transaction_v1(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 13).unwrap()),
            mock_transaction_v2(Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 12).unwrap()),
        ];

        // When
        sort_by_date(&mut transactions);

        // Then
        assert_eq!(transactions, expected);
    }
}
