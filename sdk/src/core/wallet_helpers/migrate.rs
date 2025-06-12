use etopay_wallet::VersionedWalletTransaction;

use crate::error::Result;
use crate::wallet_manager::WalletBorrow;

/// Migrate outdated transactions to their latest version.
///
/// This function checks the provided list of transactions for any that are outdated.
/// For each outdated transaction, it attempts to fetch the latest version using `wallet.get_wallet_tx`.
/// If successful, the transaction is updated; otherwise, the original (outdated) transaction is retained.
///
/// # Returns
///
/// Returns `Ok(Vec<VersionedWalletTransaction>)` containing the same set of transactions as provided,
/// with entries migrated to the latest version where possible.
pub async fn migrate_transactions(
    wallet: &WalletBorrow<'_>,
    transactions: &[VersionedWalletTransaction],
) -> Result<Vec<VersionedWalletTransaction>> {
    let mut migrated_transactions = Vec::new();
    for t in transactions {
        match t {
            VersionedWalletTransaction::V1(v1) => match wallet.get_wallet_tx(&v1.transaction_hash).await {
                Ok(details) => migrated_transactions.push(VersionedWalletTransaction::from(details)),
                Err(_) => {
                    migrated_transactions.push(t.clone());
                }
            },
            _ => migrated_transactions.push(t.clone()),
        }
    }

    Ok(migrated_transactions)
}

#[cfg(test)]
mod tests {
    use super::migrate_transactions;
    use crate::wallet_manager::WalletBorrow;
    use chrono::{DateTime, Utc};
    use etopay_wallet::{MockWalletUser, VersionedWalletTransaction, WalletTransaction, types::CryptoAmount};
    use rust_decimal::Decimal;

    fn get_outdated_transaction(date: DateTime<Utc>) -> VersionedWalletTransaction {
        VersionedWalletTransaction::V1(etopay_wallet::types::WalletTxInfoV1 {
            date,
            block_number_hash: None,
            transaction_hash: String::from("0x000"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::new(),
            status: etopay_wallet::types::WalletTxStatus::Confirmed,
            explorer_url: None,
        })
    }

    fn get_migrated_transaction(date: DateTime<Utc>) -> VersionedWalletTransaction {
        VersionedWalletTransaction::V2(etopay_wallet::types::WalletTxInfoV2 {
            date,
            block_number_hash: None,
            transaction_hash: String::from("0x000"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::new(),
            status: etopay_wallet::types::WalletTxStatus::Confirmed,
            explorer_url: None,
            gas_fee: Some(Decimal::from(1)),
            is_sender: true,
        })
    }

    #[tokio::test]
    async fn migrate_transactions_should_migrate_transaction() {
        // Given
        let date = Utc::now();
        let outdated_transactions = vec![get_outdated_transaction(date)];
        let expected = vec![get_migrated_transaction(date)];

        let mut mock_wallet_user = MockWalletUser::new();
        mock_wallet_user
            .expect_get_wallet_tx()
            .once()
            .returning(move |_| Ok(WalletTransaction::from(get_migrated_transaction(date))));

        let wallet = WalletBorrow::from(mock_wallet_user);

        // When
        let migrated_transactions = migrate_transactions(&wallet, &outdated_transactions).await.unwrap();

        // Then
        assert_eq!(expected, migrated_transactions)
    }

    #[tokio::test]
    async fn migrate_transactions_should_not_migrate_when_get_wallet_tx_fails() {
        // Given
        let date = Utc::now();
        let outdated_transactions = vec![get_outdated_transaction(date)];

        let mut mock_wallet_user = MockWalletUser::new();
        mock_wallet_user.expect_get_wallet_tx().once().returning(move |_| {
            Err(etopay_wallet::WalletError::AlloyTransportRpcError(
                alloy_transport::RpcError::NullResp,
            ))
        });

        let wallet = WalletBorrow::from(mock_wallet_user);

        // When
        let not_migrated_transactions = migrate_transactions(&wallet, &outdated_transactions).await.unwrap();

        // Then
        assert_eq!(outdated_transactions, not_migrated_transactions)
    }
}
