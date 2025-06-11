use etopay_wallet::VersionedWalletTransaction;

pub fn get_transaction_slice(
    network_key: String,
    transactions: &[VersionedWalletTransaction],
    start: usize,
    limit: usize,
) -> Vec<VersionedWalletTransaction> {
    transactions
        .iter()
        .filter(|t| match t {
            VersionedWalletTransaction::V1(w) => w.network_key == network_key,
            VersionedWalletTransaction::V2(w) => w.network_key == network_key,
        })
        .skip(start)
        .take(limit)
        .cloned()
        .collect()
}
