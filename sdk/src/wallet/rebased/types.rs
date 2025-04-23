use std::fmt;

use fastcrypto::encoding::Encoding;
use hex::FromHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs, serde_as};

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

impl fmt::Display for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", fastcrypto::encoding::Hex::encode(self.0))
    }
}

#[serde_as]
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectID(
    // #[serde_as(as = "Readable<HexAccountAddress, _>")]
    #[serde_as(as = "HexAccountAddress")] AccountAddress,
);

/// temporary implementation to ease impl
impl From<ObjectID> for iota_sdk_rebased::types::base_types::ObjectID {
    fn from(value: ObjectID) -> Self {
        Self::new(value.0.0)
    }
}

pub struct HexAccountAddress;

impl SerializeAs<AccountAddress> for HexAccountAddress {
    fn serialize_as<S>(value: &AccountAddress, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fastcrypto::encoding::Hex::serialize_as(value, serializer)
    }
}

impl<'de> DeserializeAs<'de, AccountAddress> for HexAccountAddress {
    fn deserialize_as<D>(deserializer: D) -> Result<AccountAddress, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("0x") {
            AccountAddress::from_hex_literal(&s)
        } else {
            AccountAddress::from_hex(&s)
        }
        .map_err(|e| serde::de::Error::custom(format!("byte deserialization failed, cause by: {:?}", e)))
    }
}

/// A struct that represents an account address.
/// (This is from move-core-types)
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
pub struct AccountAddress([u8; AccountAddress::LENGTH]);

#[derive(Clone, Copy, Debug)]
pub struct AccountAddressParseError;

impl AccountAddress {
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    /// The number of bytes in an address.
    pub const LENGTH: usize = 32;

    pub fn from_hex_literal(literal: &str) -> Result<Self, AccountAddressParseError> {
        if !literal.starts_with("0x") {
            return Err(AccountAddressParseError);
        }

        let hex_len = literal.len() - 2;

        // If the string is too short, pad it
        if hex_len < Self::LENGTH * 2 {
            let mut hex_str = String::with_capacity(Self::LENGTH * 2);
            for _ in 0..Self::LENGTH * 2 - hex_len {
                hex_str.push('0');
            }
            hex_str.push_str(&literal[2..]);
            AccountAddress::from_hex(hex_str)
        } else {
            AccountAddress::from_hex(&literal[2..])
        }
    }

    pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, AccountAddressParseError> {
        <[u8; Self::LENGTH]>::from_hex(hex)
            .map_err(|_| AccountAddressParseError)
            .map(Self)
    }
}

impl AsRef<[u8]> for AccountAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

// impl From<[u8; AccountAddress::LENGTH]> for AccountAddress {
//     fn from(bytes: [u8; AccountAddress::LENGTH]) -> Self {
//         Self::new(bytes)
//     }
// }
//
// impl From<&AccountAddress> for [u8; AccountAddress::LENGTH] {
//     fn from(addr: &AccountAddress) -> Self {
//         addr.0
//     }
// }

impl fmt::Debug for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}
impl fmt::LowerHex for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}
