// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/write.rs

// use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::rebased::RpcClient;
use crate::rebased::client::{RpcResponse, RpcResult};

use super::super::encoding::Base64;
use super::{DryRunTransactionBlockResponse, IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions};

/// Provides methods for executing and testing transactions.
// #[rpc(client, namespace = "iota")]
pub trait WriteApi {
    /// Execute the transaction and wait for results if desired.
    /// Request types:
    /// 1. WaitForEffectsCert: waits for TransactionEffectsCert and then return to client.
    ///     This mode is a proxy for transaction finality.
    /// 2. WaitForLocalExecution: waits for TransactionEffectsCert and make sure the node
    ///     executed the transaction locally before returning the client. The local execution
    ///     makes sure this node is aware of this transaction when client fires subsequent queries.
    ///     However if the node fails to execute the transaction locally in a timely manner,
    ///     a bool type in the response is set to false to indicated the case.
    /// request_type is default to be `WaitForEffectsCert` unless options.show_events or options.show_effects is true
    // #[rustfmt::skip]
    // #[method(name = "executeTransactionBlock")]
    async fn execute_transaction_block(
        &self,
        // BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
        tx_bytes: Base64,
        // A list of signatures (`flag || signature || pubkey` bytes, as base-64 encoded string). Signature is committed to the intent message of the transaction data, as base-64 encoded string.
        signatures: Vec<Base64>,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
        // The request type, derived from `IotaTransactionBlockResponseOptions` if None
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<IotaTransactionBlockResponse>;
    /// Return transaction execution effects including the gas cost summary,
    /// while the effects are not committed to the chain.
    // #[method(name = "dryRunTransactionBlock")]
    async fn dry_run_transaction_block(&self, tx_bytes: Base64) -> RpcResult<DryRunTransactionBlockResponse>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ExecuteTransactionRequestType {
    WaitForEffectsCert,
    WaitForLocalExecution,
}

impl WriteApi for RpcClient {
    async fn execute_transaction_block(
        &self,
        // BCS serialized transaction data bytes without its type tag, as base-64 encoded string.
        tx_bytes: Base64,
        // A list of signatures (`flag || signature || pubkey` bytes, as base-64 encoded string). Signature is committed to the intent message of the transaction data, as base-64 encoded string.
        signatures: Vec<Base64>,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
        // The request type, derived from `IotaTransactionBlockResponseOptions` if None
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<IotaTransactionBlockResponse> {
        let mut params: Vec<Value> = vec![json!(tx_bytes), json!(signatures)];

        if let Some(opts) = options {
            params.push(json!(opts));
        }

        if let Some(req_type) = request_type {
            params.push(json!(req_type));
        }

        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "iota_executeTransactionBlock",
            "params": params
        });

        let response = self
            .client
            .post("https://api.testnet.iota.cafe")
            .json(&request_body)
            .send()
            .await?;

        Ok(response
            .json::<RpcResponse<IotaTransactionBlockResponse>>()
            .await?
            .result)
    }

    async fn dry_run_transaction_block(&self, tx_bytes: Base64) -> RpcResult<DryRunTransactionBlockResponse> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "iota_dryRunTransactionBlock",
            "params": [
                json!(tx_bytes)
            ]
        });

        let response = self
            .client
            .post("https://api.testnet.iota.cafe")
            .json(&request_body)
            .send()
            .await?;

        Ok(response
            .json::<RpcResponse<DryRunTransactionBlockResponse>>()
            .await?
            .result)
    }
}
