// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use serde_json::json;

use crate::rebased::{
    RpcClient,
    client::{RawRpcResponse, RpcResult},
};

use super::super::bigint::BigInt;

/// Provides access to validator and staking-related data such as current
/// committee info, delegated stakes, and APY.
pub trait GovernanceReadApi {
    /// Return the reference gas price for the network
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;
}

impl GovernanceReadApi for RpcClient {
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "iotax_getReferenceGasPrice",
            "params": []
        });

        let response = self.client.post(self.url.clone()).json(&request_body).send().await?;

        let body: RawRpcResponse<BigInt<u64>> = response.json().await?;
        body.into_result()
    }
}
