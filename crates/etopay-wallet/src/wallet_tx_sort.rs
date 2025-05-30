use chrono::DateTime;
use std::cmp::Ordering;

use crate::types::WalletTxInfo;

pub fn sort_by_date(transactions: &mut [WalletTxInfo]) {
    transactions.sort_by(|a, b| {
        let a_date = DateTime::parse_from_rfc3339(&a.date);
        let b_date = DateTime::parse_from_rfc3339(&b.date);

        match (a_date, b_date) {
            (Ok(a_dt), Ok(b_dt)) => b_dt.cmp(&a_dt),
            // fallback if parsing fails: put unparsable dates at the end
            (Ok(_), Err(_)) => Ordering::Less,
            (Err(_), Ok(_)) => Ordering::Greater,
            (Err(_), Err(_)) => Ordering::Equal,
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::types::{CryptoAmount, WalletTxInfo};

    use super::sort_by_date;

    fn mock_transaction(date: &str) -> WalletTxInfo {
        WalletTxInfo {
            date: String::from(date),
            block_number_hash: None,
            transaction_hash: String::from("tx_hash"),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(1),
            network_key: String::from("network"),
            status: crate::types::WalletTxStatus::Pending,
            explorer_url: None,
        }
    }

    #[test]
    fn test_should_sort_by_date() {
        // Given
        let mut transactions = vec![
            mock_transaction("2025-05-29T08:37:15.183+00:00"),
            mock_transaction("2025-05-29T08:37:14.183+00:00"),
            mock_transaction("2025-05-29T08:37:13.183+00:00"),
            mock_transaction("2025-05-29T08:37:16.183+00:00"),
            mock_transaction("2025-05-29T08:37:12.183+00:00"),
        ];

        let expected = vec![
            mock_transaction("2025-05-29T08:37:16.183+00:00"),
            mock_transaction("2025-05-29T08:37:15.183+00:00"),
            mock_transaction("2025-05-29T08:37:14.183+00:00"),
            mock_transaction("2025-05-29T08:37:13.183+00:00"),
            mock_transaction("2025-05-29T08:37:12.183+00:00"),
        ];

        // When
        sort_by_date(&mut transactions);

        // Then
        assert_eq!(expected, transactions);
    }

    #[test]
    fn test_should_put_unparsable_dates_at_the_end() {
        // Given
        let mut transactions = vec![
            mock_transaction("2026-05-29"),
            mock_transaction("invalid date"),
            mock_transaction("2025-05-29T08:37:14.183+00:00"),
            mock_transaction("2025-05-29T08:37:13.183+00:00"),
        ];

        let expected = vec![
            mock_transaction("2025-05-29T08:37:14.183+00:00"),
            mock_transaction("2025-05-29T08:37:13.183+00:00"),
            mock_transaction("2026-05-29"),
            mock_transaction("invalid date"),
        ];

        // When
        sort_by_date(&mut transactions);

        // Then
        assert_eq!(expected, transactions);
    }
}
