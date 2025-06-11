use etopay_wallet::VersionedWalletTransaction;

/// Merge incoming transactions into the current list.
///
/// This function compares `incoming_transactions` with `current_transactions`
/// using `transaction_hash` and `network_key` as identifiers. If a matching transaction
/// is found in `current_transactions`, it is updated with the new data. If no match is found,
/// the transaction is added to the list.
///
/// # Returns
///
/// This function does not return a value. The `current_transactions` vector is modified in place.
pub fn merge_transactions(
    current_transactions: &mut Vec<VersionedWalletTransaction>,
    incoming_transactions: Vec<VersionedWalletTransaction>,
) {
    for tx in incoming_transactions {
        let (hash, network_key) = match &tx {
            VersionedWalletTransaction::V1(v1) => (&v1.transaction_hash, &v1.network_key),
            VersionedWalletTransaction::V2(v2) => (&v2.transaction_hash, &v2.network_key),
        };

        if let Some(existing) = current_transactions.iter_mut().find(|tx| match tx {
            VersionedWalletTransaction::V1(v1) => v1.transaction_hash == *hash && v1.network_key == *network_key,
            VersionedWalletTransaction::V2(v2) => v2.transaction_hash == *hash && v2.network_key == *network_key,
        }) {
            *existing = tx;
        } else {
            current_transactions.push(tx)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::merge_transactions;
    use chrono::{DateTime, Utc};
    use etopay_wallet::{
        VersionedWalletTransaction,
        types::{CryptoAmount, WalletTxInfoV2, WalletTxStatus},
    };
    use rust_decimal::Decimal;

    fn get_mocked_date() -> DateTime<Utc> {
        let date = "2024-06-01T12:00:00Z";
        date.parse().unwrap()
    }

    fn get_transaction(transaction_hash: String, gas_fee: Option<Decimal>) -> VersionedWalletTransaction {
        VersionedWalletTransaction::V2(WalletTxInfoV2 {
            date: get_mocked_date(),
            block_number_hash: None,
            transaction_hash,
            receiver: String::new(),
            sender: String::new(),
            amount: CryptoAmount::from(1),
            network_key: "ETH".to_string(),
            status: WalletTxStatus::Confirmed,
            explorer_url: None,
            gas_fee,
        })
    }

    #[tokio::test]
    async fn test_merge_transactions_should_append_new_transactions() {
        // Arrange
        let tx_1 = get_transaction(String::from("1"), None);
        let tx_2 = get_transaction(String::from("2"), None);
        let tx_3 = get_transaction(String::from("3"), None);

        let mut wallet_transactions = vec![tx_1.clone(), tx_2.clone()];
        let new_transactions = vec![tx_3.clone()];
        let expected = vec![tx_1, tx_2, tx_3];

        // Act
        merge_transactions(&mut wallet_transactions, new_transactions);

        // Assert
        assert_eq!(expected, wallet_transactions)
    }

    #[tokio::test]
    async fn test_merge_transactions_should_update_existing_transactions() {
        // Arrange
        let tx_1 = get_transaction(String::from("1"), None);
        let tx_1_updated = get_transaction(String::from("1"), Some(Decimal::from(3)));

        let mut wallet_transactions = vec![tx_1.clone()];

        let new_transactions = vec![tx_1_updated.clone()];
        let expected = vec![tx_1_updated];

        // Act
        merge_transactions(&mut wallet_transactions, new_transactions);

        // Assert
        assert_eq!(expected, wallet_transactions)
    }
}
