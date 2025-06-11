use crate::error::Result;
use etopay_wallet::{VersionedWalletTransaction, WalletTransaction};

use crate::wallet_manager::WalletBorrow;

pub async fn fetch_new_hashes_and_turn_into_versioned_transactions(
    wallet: &WalletBorrow<'_>,
    wallet_transactions: &[VersionedWalletTransaction],
    network_key: String,
    start: usize,
    limit: usize,
) -> Result<Vec<VersionedWalletTransaction>> {
    let hashes = fetch_hashes_from_network(wallet, start, limit).await?;
    let new_hashes = filter_new_hashes(hashes, network_key, wallet_transactions);
    let new_transactions = turn_new_hashes_into_transactions(wallet, &new_hashes).await?;
    let new_versioned_transactions = new_transactions
        .into_iter()
        .map(VersionedWalletTransaction::from)
        .collect();
    Ok(new_versioned_transactions)
}

async fn fetch_hashes_from_network(wallet: &WalletBorrow<'_>, start: usize, limit: usize) -> Result<Vec<String>> {
    match wallet.get_wallet_tx_list(start, limit).await {
        Ok(transaction_hashes) => Ok(transaction_hashes),
        // do nothing if feature is not supported
        Err(etopay_wallet::WalletError::WalletFeatureNotImplemented) => Ok(Vec::new()),
        Err(e) => Err(e.into()),
    }
}

fn filter_new_hashes(
    transaction_hashes: Vec<String>,
    network_key: String,
    wallet_transactions: &[VersionedWalletTransaction],
) -> Vec<String> {
    let mut new_hashes = Vec::new();
    // go through and get the details for any new hashes
    log::debug!("Digests: {:#?}", transaction_hashes);
    for hash in transaction_hashes {
        // check if transaction is already in the list (not very efficient to do a linear search, but good enough for now)
        // check both the transaction hash and the network key, as hash collisions can occur across different blockchain networks
        if wallet_transactions.iter().any(|t| match t {
            VersionedWalletTransaction::V1(v1) => v1.transaction_hash == hash && v1.network_key == network_key,
            VersionedWalletTransaction::V2(v2) => v2.transaction_hash == hash && v2.network_key == network_key,
        }) {
            continue;
        }

        new_hashes.push(hash)
    }

    new_hashes
}

async fn turn_new_hashes_into_transactions(
    wallet: &WalletBorrow<'_>,
    hashes: &[String],
) -> Result<Vec<WalletTransaction>> {
    let mut transactions = Vec::with_capacity(hashes.len());
    for hash in hashes {
        let tx = wallet.get_wallet_tx(hash).await?;
        transactions.push(tx);
    }

    Ok(transactions)
}
