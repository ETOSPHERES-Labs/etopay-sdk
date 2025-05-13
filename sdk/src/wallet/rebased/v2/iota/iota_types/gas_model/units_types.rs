// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    ops::{Add, Bound},
};

use crate::wallet::rebased::v2::mowe::move_core_types::gas_algebra::GasQuantity;

pub const STACK_HEIGHT_TIER_DEFAULT: u64 = 1;
pub const INSTRUCTION_TIER_DEFAULT: u64 = 1;
pub const STACK_SIZE_TIER_DEFAULT: u64 = 1;

// The cost table holds the tiers and curves for instruction costs.
#[derive(Clone, Debug, Serialize, PartialEq, Eq, Deserialize)]
pub struct CostTable {
    pub instruction_tiers: BTreeMap<u64, u64>,
    pub stack_height_tiers: BTreeMap<u64, u64>,
    pub stack_size_tiers: BTreeMap<u64, u64>,
}

impl CostTable {
    fn get_current_and_future_tier(tiers: &BTreeMap<u64, u64>, current: u64, default: u64) -> (u64, Option<u64>) {
        let current_cost = tiers
            .get(&current)
            .or_else(|| tiers.range(..current).next_back().map(|(_, v)| v))
            .unwrap_or(&default);
        let next_tier_start = tiers
            .range::<u64, _>((Bound::Excluded(current), Bound::Unbounded))
            .next()
            .map(|(next_tier_start, _)| *next_tier_start);
        (*current_cost, next_tier_start)
    }

    pub fn stack_height_tier(&self, stack_height: u64) -> (u64, Option<u64>) {
        Self::get_current_and_future_tier(&self.stack_height_tiers, stack_height, STACK_HEIGHT_TIER_DEFAULT)
    }

    pub fn stack_size_tier(&self, stack_size: u64) -> (u64, Option<u64>) {
        Self::get_current_and_future_tier(&self.stack_size_tiers, stack_size, STACK_SIZE_TIER_DEFAULT)
    }

    pub fn instruction_tier(&self, instr_count: u64) -> (u64, Option<u64>) {
        Self::get_current_and_future_tier(&self.instruction_tiers, instr_count, INSTRUCTION_TIER_DEFAULT)
    }
}

pub enum GasUnit {}

pub type Gas = GasQuantity<GasUnit>;
