// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::proc_macros::rpc;

use super::super::bigint::BigInt;

/// Provides access to validator and staking-related data such as current
/// committee info, delegated stakes, and APY.
#[rpc(client, namespace = "iotax")]
pub trait GovernanceReadApi {
    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;
}
