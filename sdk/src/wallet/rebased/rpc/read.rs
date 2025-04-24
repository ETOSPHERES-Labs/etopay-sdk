// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/read.rs

use jsonrpsee::proc_macros::rpc;

use super::{super::TransactionDigest, IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions};

/// Provides methods for reading transaction related data such as transaction
/// blocks, checkpoints, and protocol configuration. The trait further provides
/// methods for reading the ledger (current objects) as well its history (past
/// objects).
#[rpc(client, namespace = "iota")]
pub trait ReadApi {
    /// Return the transaction response object.
    #[rustfmt::skip]
    #[method(name = "getTransactionBlock")]
    async fn get_transaction_block(
        &self,
        // the digest of the queried transaction
        digest: TransactionDigest,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
    ) -> RpcResult<IotaTransactionBlockResponse>;
}
