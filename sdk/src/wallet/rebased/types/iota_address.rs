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

use super::super::serde::Readable;

/// An address formatted as a string

pub const IOTA_ADDRESS_LENGTH: usize = 32;

pub enum IotaError {
    InvalidAddress,
    KeyConversion(String),
}

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct IotaAddress(#[serde_as(as = "Readable<Hex, _>")] pub [u8; IOTA_ADDRESS_LENGTH]);

impl IotaAddress {
    /// Parse a IotaAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, IotaError> {
        <[u8; IOTA_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| IotaError::InvalidAddress)
            .map(IotaAddress)
    }
}

impl TryFrom<&[u8]> for IotaAddress {
    type Error = IotaError;

    /// Tries to convert the provided byte array into a IotaAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, IotaError> {
        Self::from_bytes(bytes)
    }
}

impl FromStr for IotaAddress {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fastcrypto::encoding::decode_bytes_hex(s).map_err(|e| anyhow::anyhow!(e))
    }
}

/// temporary implementation to ease impl
impl From<iota_sdk_rebased::types::base_types::IotaAddress> for IotaAddress {
    fn from(value: iota_sdk_rebased::types::base_types::IotaAddress) -> Self {
        Self(value.to_inner())
    }
}

impl From<IotaAddress> for iota_sdk_rebased::types::base_types::IotaAddress {
    fn from(value: IotaAddress) -> Self {
        Self::new(value.0)
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
