use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// An address formatted as a string

pub const IOTA_ADDRESS_LENGTH: usize = 32;

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize)]
pub struct IotaAddress(
    // #[serde_as(as = "Readable<Hex, _>")]
    #[serde_as(as = "serde_with::hex::Hex")] pub [u8; IOTA_ADDRESS_LENGTH],
);

/// temporary implementation to ease impl
impl From<iota_sdk_rebased::types::base_types::IotaAddress> for IotaAddress {
    fn from(value: iota_sdk_rebased::types::base_types::IotaAddress) -> Self {
        Self(value.to_inner())
    }
}
