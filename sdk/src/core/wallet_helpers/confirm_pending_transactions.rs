use crate::error::Result;
use crate::wallet_manager::WalletBorrow;
use etopay_wallet::{VersionedWalletTransaction, types::WalletTxStatus};

pub async fn confirm_pending_transactions(
    wallet: &WalletBorrow<'_>,
    transactions: &Vec<VersionedWalletTransaction>,
) -> Result<Vec<VersionedWalletTransaction>> {
    let mut result = Vec::with_capacity(transactions.len());
    for t in transactions {
        match t {
            VersionedWalletTransaction::V1(v1) => match v1.status {
                WalletTxStatus::Pending => match wallet.get_wallet_tx(&v1.transaction_hash).await {
                    Ok(details) => result.push(VersionedWalletTransaction::V2(details)),
                    Err(_) => {
                        result.push(t.clone());
                    }
                },
                _ => result.push(t.clone()),
            },
            VersionedWalletTransaction::V2(v2) => match v2.status {
                WalletTxStatus::Pending => match wallet.get_wallet_tx(&v2.transaction_hash).await {
                    Ok(details) => result.push(VersionedWalletTransaction::V2(details)),
                    Err(_) => {
                        result.push(t.clone());
                    }
                },
                _ => result.push(t.clone()),
            },
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::confirm_pending_transactions;
    use crate::wallet_manager::WalletBorrow;
    use chrono::Utc;
    use etopay_wallet::{MockWalletUser, VersionedWalletTransaction, WalletTransaction, types::CryptoAmount};
    use rust_decimal::Decimal;

    fn get_pending_transaction() -> VersionedWalletTransaction {
        VersionedWalletTransaction::V2(etopay_wallet::types::WalletTxInfoV2 {
            date: Utc::now(),
            block_number_hash: None,
            transaction_hash: String::from("0x000"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::new(),
            status: etopay_wallet::types::WalletTxStatus::Pending,
            explorer_url: None,
            gas_fee: Some(Decimal::from(1)),
        })
    }

    fn get_confirmed_transaction() -> VersionedWalletTransaction {
        VersionedWalletTransaction::V2(etopay_wallet::types::WalletTxInfoV2 {
            date: Utc::now(),
            block_number_hash: None,
            transaction_hash: String::from("0x000"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::new(),
            status: etopay_wallet::types::WalletTxStatus::Confirmed,
            explorer_url: None,
            gas_fee: Some(Decimal::from(1)),
        })
    }

    #[tokio::test]
    async fn confirm_pending_transactions_should_update_transaction() {
        // Given
        let pending_transactions = vec![get_pending_transaction()];
        let expected = vec![get_confirmed_transaction()];

        let mut mock_wallet_user = MockWalletUser::new();
        mock_wallet_user
            .expect_get_wallet_tx()
            .once()
            .returning(move |_| Ok(WalletTransaction::from(get_confirmed_transaction())));

        let wallet = WalletBorrow::from(mock_wallet_user);

        // When
        let result = confirm_pending_transactions(&wallet, &pending_transactions)
            .await
            .unwrap();

        // Then
        assert_eq!(expected, result)
    }

    #[tokio::test]
    async fn confirm_pending_transactions_should_update_and_migrate_if_current_version_is_outdated() {
        // Given
        let pending_outdated_transaction = VersionedWalletTransaction::V1(etopay_wallet::types::WalletTxInfoV1 {
            date: Utc::now(),
            block_number_hash: None,
            transaction_hash: String::from("0x000"),
            sender: String::from("Satoshi"),
            receiver: String::from("Bob"),
            amount: CryptoAmount::from(3),
            network_key: String::new(),
            status: etopay_wallet::types::WalletTxStatus::Pending,
            explorer_url: None,
        });

        let pending_transactions = vec![pending_outdated_transaction];
        let expected = vec![get_confirmed_transaction()];

        let mut mock_wallet_user = MockWalletUser::new();
        mock_wallet_user
            .expect_get_wallet_tx()
            .once()
            .returning(move |_| Ok(WalletTransaction::from(get_confirmed_transaction())));

        let wallet = WalletBorrow::from(mock_wallet_user);

        // When
        let result = confirm_pending_transactions(&wallet, &pending_transactions)
            .await
            .unwrap();

        // Then
        assert_eq!(expected, result)
    }

    #[tokio::test]
    async fn confirm_pending_transactions_should_not_update_when_get_wallet_tx_fails() {
        // Given
        let pending_transactions = vec![get_pending_transaction()];
        let expected = vec![get_pending_transaction()];

        let mut mock_wallet_user = MockWalletUser::new();
        mock_wallet_user.expect_get_wallet_tx().once().returning(move |_| {
            Err(etopay_wallet::WalletError::AlloyTransportRpcError(
                alloy_transport::RpcError::NullResp,
            ))
        });

        let wallet = WalletBorrow::from(mock_wallet_user);

        // When
        let not_updated_transactions = confirm_pending_transactions(&wallet, &pending_transactions)
            .await
            .unwrap();

        // Then
        assert_eq!(expected, not_updated_transactions)
    }
}
