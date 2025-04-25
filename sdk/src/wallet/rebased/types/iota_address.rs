// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/base_types.rs#L642

use std::{fmt, str::FromStr};

use fastcrypto::encoding::{Encoding, Hex};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::super::RebasedError;
use super::super::serde::Readable;

/// An address formatted as a string

pub const IOTA_ADDRESS_LENGTH: usize = 32;

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct IotaAddress(#[serde_as(as = "Readable<Hex, _>")] pub [u8; IOTA_ADDRESS_LENGTH]);

impl IotaAddress {
    /// Parse a IotaAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, RebasedError> {
        <[u8; IOTA_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| RebasedError::InvalidAddress)
            .map(IotaAddress)
    }
}

impl TryFrom<&[u8]> for IotaAddress {
    type Error = RebasedError;

    /// Tries to convert the provided byte array into a IotaAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, RebasedError> {
        Self::from_bytes(bytes)
    }
}

impl FromStr for IotaAddress {
    type Err = RebasedError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(fastcrypto::encoding::decode_bytes_hex(s)?)
    }
}

impl fmt::Display for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", fastcrypto::encoding::Hex::encode(self.0))
    }
}

impl fmt::Debug for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", fastcrypto::encoding::Hex::encode(self.0))
    }
}
