// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use super::super::bigint::BigInt;
use super::super::serde::Readable;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GasCostSummary {
    /// Cost of computation/execution
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub computation_cost: u64,
    /// The burned component of the computation/execution costs
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub computation_cost_burned: u64,
    /// Storage cost, it's the sum of all storage cost for all objects
    /// created or mutated.
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub storage_cost: u64,
    /// The amount of storage cost refunded to the user for all objects
    /// deleted or mutated in the transaction.
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub storage_rebate: u64,
    /// The fee for the rebate. The portion of the storage rebate kept by
    /// the system.
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub non_refundable_storage_fee: u64,
}
