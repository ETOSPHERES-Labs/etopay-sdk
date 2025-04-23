use std::fmt;

use fastcrypto::encoding::Encoding;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::super::serde::Readable;

/// An address formatted as a string

pub const IOTA_ADDRESS_LENGTH: usize = 32;

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct IotaAddress(#[serde_as(as = "Readable<serde_with::hex::Hex, _>")] pub [u8; IOTA_ADDRESS_LENGTH]);

/// temporary implementation to ease impl
impl From<iota_sdk_rebased::types::base_types::IotaAddress> for IotaAddress {
    fn from(value: iota_sdk_rebased::types::base_types::IotaAddress) -> Self {
        Self(value.to_inner())
    }
}

impl fmt::Display for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", fastcrypto::encoding::Hex::encode(self.0))
    }
}
