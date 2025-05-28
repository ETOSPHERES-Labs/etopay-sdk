// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/indexer.rs

use serde_json::json;

use crate::rebased::{
    RpcClient,
    client::{RawRpcResponse, RpcResult},
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{
    super::{IotaAddress, TransactionDigest},
    IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions,
};

/// Provides methods to query transactions, events, or objects and allows to
/// subscribe to data streams.
pub trait IndexerApi {
    /// Return list of transactions for a specified query criteria.
    #[rustfmt::skip]
    // #[method(name = "queryTransactionBlocks")]
    async fn query_transaction_blocks(
        &self,
        // the transaction query criteria.
        query: IotaTransactionBlockResponseQuery,
        // An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<TransactionDigest>,
        // Maximum item returned per page, default to QUERY_MAX_RESULT_LIMIT if not specified.
        limit: Option<usize>,
        // query result ordering, default to false (ascending order), oldest record first.
        descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage>;
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase", rename = "TransactionBlockResponseQuery", default)]
pub struct IotaTransactionBlockResponseQuery {
    /// If None, no filter will be applied
    pub filter: Option<TransactionFilter>,
    /// config which fields to include in the response, by default only digest
    /// is included
    pub options: Option<IotaTransactionBlockResponseOptions>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TransactionFilter {
    // /// Query by checkpoint.
    // Checkpoint(
    //     #[schemars(with = "BigInt<u64>")]
    //     #[serde_as(as = "Readable<BigInt<u64>, _>")]
    //     CheckpointSequenceNumber,
    // ),
    // /// Query by move function.
    // MoveFunction {
    //     package: ObjectID,
    //     module: Option<String>,
    //     function: Option<String>,
    // },
    // /// Query by input object.
    // InputObject(ObjectID),
    // /// Query by changed object, including created, mutated and unwrapped
    // /// objects.
    // ChangedObject(ObjectID),
    /// Query by sender address.
    FromAddress(IotaAddress),
    /// Query by recipient address.
    ToAddress(IotaAddress),
    // /// Query by sender and recipient address.
    // FromAndToAddress { from: IotaAddress, to: IotaAddress },
    // /// Query txs that have a given address as sender or recipient.
    // /// Note: only supported by the indexer! (different URL)
    // FromOrToAddress { addr: IotaAddress },
    // /// Query by transaction kind
    // TransactionKind(IotaTransactionKind),
    // /// Query transactions of any given kind in the input.
    // TransactionKindIn(Vec<IotaTransactionKind>),
}

pub type TransactionBlocksPage = super::Page<IotaTransactionBlockResponse, TransactionDigest>;

impl IndexerApi for RpcClient {
    async fn query_transaction_blocks(
        &self,
        // the transaction query criteria.
        query: IotaTransactionBlockResponseQuery,
        // An optional paging cursor. If provided, the query will start from the next item after the specified cursor. Default to start from the first item if not specified.
        cursor: Option<TransactionDigest>,
        // Maximum item returned per page, default to QUERY_MAX_RESULT_LIMIT if not specified.
        limit: Option<usize>,
        // query result ordering, default to false (ascending order), oldest record first.
        descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "iotax_queryTransactionBlocks",
            "params": [
                json!(query),
                json!(cursor),
                json!(limit),
                json!(descending_order)
            ],
        });

        let response = self.client.post(self.url.clone()).json(&request_body).send().await?;

        let body: RawRpcResponse<TransactionBlocksPage> = response.json().await?;
        body.into_result()
    }
}
