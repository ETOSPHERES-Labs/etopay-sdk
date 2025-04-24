// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use fastcrypto::encoding::{Encoding, Hex};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{super::serde::Readable, AccountAddress, HexAccountAddress, ObjectDigest, SequenceNumber};

#[serde_as]
#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectID(#[serde_as(as = "Readable<HexAccountAddress, _>")] AccountAddress);

impl fmt::Display for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl fmt::Debug for ObjectID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

pub type ObjectRef = (ObjectID, SequenceNumber, ObjectDigest);
