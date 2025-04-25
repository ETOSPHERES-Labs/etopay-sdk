// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Default, Debug, Serialize, Deserialize)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub const MIN: SequenceNumber = SequenceNumber(u64::MIN);
    pub const MAX: SequenceNumber = SequenceNumber(0x7fff_ffff_ffff_ffff);
    pub const CANCELLED_READ: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 1);
    pub const CONGESTED: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 2);
    pub const RANDOMNESS_UNAVAILABLE: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 3);

    pub const fn new() -> Self {
        SequenceNumber(0)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn from_u64(u: u64) -> Self {
        SequenceNumber(u)
    }

    pub fn increment(&mut self) {
        assert_ne!(self.0, u64::MAX);
        self.0 += 1;
    }

    pub fn increment_to(&mut self, next: SequenceNumber) {
        debug_assert!(*self < next, "Not an increment: {} to {}", self, next);
        *self = next;
    }

    pub fn decrement(&mut self) {
        assert_ne!(self.0, 0);
        self.0 -= 1;
    }

    pub fn decrement_to(&mut self, prev: SequenceNumber) {
        debug_assert!(prev < *self, "Not a decrement: {} to {}", self, prev);
        *self = prev;
    }

    pub fn is_cancelled(&self) -> bool {
        self == &SequenceNumber::CANCELLED_READ
            || self == &SequenceNumber::CONGESTED
            || self == &SequenceNumber::RANDOMNESS_UNAVAILABLE
    }

    pub fn is_valid(&self) -> bool {
        self < &SequenceNumber::MAX
    }
}

impl From<SequenceNumber> for u64 {
    fn from(val: SequenceNumber) -> Self {
        val.0
    }
}

impl From<u64> for SequenceNumber {
    fn from(value: u64) -> Self {
        SequenceNumber(value)
    }
}

impl From<SequenceNumber> for usize {
    fn from(value: SequenceNumber) -> Self {
        value.0 as usize
    }
}

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
