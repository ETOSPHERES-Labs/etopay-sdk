// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/read.rs

use serde_json::{Value, json};

use crate::rebased::{
    RpcClient,
    client::{RpcResponse, RpcResult},
};

use super::{super::TransactionDigest, IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions};

/// Provides methods for reading transaction related data such as transaction
/// blocks, checkpoints, and protocol configuration. The trait further provides
/// methods for reading the ledger (current objects) as well its history (past
/// objects).
pub trait ReadApi {
    /// Return the transaction response object.
    async fn get_transaction_block(
        &self,
        // the digest of the queried transaction
        digest: TransactionDigest,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
    ) -> RpcResult<IotaTransactionBlockResponse>;
}

impl ReadApi for RpcClient {
    async fn get_transaction_block(
        &self,
        // the digest of the queried transaction
        digest: TransactionDigest,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
    ) -> RpcResult<IotaTransactionBlockResponse> {
        let mut params: Vec<Value> = vec![json!(digest.to_string())];

        if let Some(opts) = options {
            params.push(json!(opts));
        }

        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "iota_getTransactionBlock",
            "params": params
        });

        let response = self.client.post(self.url.clone()).json(&request_body).send().await?;

        Ok(response
            .json::<RpcResponse<IotaTransactionBlockResponse>>()
            .await?
            .result)
    }
}
