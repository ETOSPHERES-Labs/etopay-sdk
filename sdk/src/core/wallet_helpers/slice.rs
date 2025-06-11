use etopay_wallet::VersionedWalletTransaction;

/// Return a paginated slice of transactions for a specific network.
///
/// This function filters the provided list of transactions by `network_key`,
/// then returns a paginated slice starting at the given `start` index and containing
/// up to `limit` transactions. The returned transactions are cloned from the input slice.
///
/// # Returns
///
/// Returns a `Vec<VersionedWalletTransaction>` containing the filtered and paginated transactions.
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
