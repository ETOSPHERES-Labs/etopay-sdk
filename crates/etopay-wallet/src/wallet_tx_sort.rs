use crate::VersionedWalletTransaction;

pub fn sort_wallet_transactions_by_date(transactions: &mut [VersionedWalletTransaction]) {
    transactions.sort_by(|a, b| {
        let a_date = match a {
            VersionedWalletTransaction::V1(w) => w.date,
            VersionedWalletTransaction::V2(w) => w.date,
        };

        let b_date = match b {
            VersionedWalletTransaction::V1(w) => w.date,
            VersionedWalletTransaction::V2(w) => w.date,
        };

        b_date.cmp(&a_date)
    });
}

#[cfg(test)]
mod tests {
    use super::sort_wallet_transactions_by_date;
    use crate::{
        VersionedWalletTransaction,
        types::{CryptoAmount, WalletTxInfoV1, WalletTxInfoV2, WalletTxStatus},
    };
    use chrono::{DateTime, TimeZone, Utc};

    fn mock_transaction_v2(date: DateTime<Utc>) -> VersionedWalletTransaction {
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

        VersionedWalletTransaction::V2(v2)
    }

    fn mock_transaction_v1(date: DateTime<Utc>) -> VersionedWalletTransaction {
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

        VersionedWalletTransaction::V1(v1)
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
        sort_wallet_transactions_by_date(&mut transactions);

        // Then
        assert_eq!(transactions, expected);
    }
}
