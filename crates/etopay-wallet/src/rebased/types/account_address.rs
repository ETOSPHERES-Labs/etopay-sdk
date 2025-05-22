// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use std::{fmt, str::FromStr};

use hex::FromHex;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

use super::super::encoding;

pub struct HexAccountAddress;

impl SerializeAs<AccountAddress> for HexAccountAddress {
    fn serialize_as<S>(value: &AccountAddress, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        encoding::Hex::serialize_as(value, serializer)
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

    pub fn to_hex(self) -> String {
        format!("{:x}", self)
    }

    pub fn to_inner(self) -> [u8; Self::LENGTH] {
        self.0
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

impl FromStr for AccountAddress {
    type Err = AccountAddressParseError;

    fn from_str(s: &str) -> Result<Self, AccountAddressParseError> {
        // Accept 0xADDRESS or ADDRESS
        if let Ok(address) = AccountAddress::from_hex_literal(s) {
            Ok(address)
        } else {
            Self::from_hex(s)
        }
    }
}

impl<'de> Deserialize<'de> for AccountAddress {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let s = <String>::deserialize(deserializer)?;
            AccountAddress::from_str(&s).map_err(serde::de::Error::custom)
        } else {
            // In order to preserve the Serde data model and help analysis tools,
            // make sure to wrap our value in a container with the same name
            // as the original type.
            #[derive(::serde::Deserialize)]
            #[serde(rename = "AccountAddress")]
            struct Value([u8; AccountAddress::LENGTH]);

            let value = Value::deserialize(deserializer)?;
            Ok(AccountAddress::new(value.0))
        }
    }
}

impl Serialize for AccountAddress {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            self.to_hex().serialize(serializer)
        } else {
            // See comment in deserialize.
            serializer.serialize_newtype_struct("AccountAddress", &self.0)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AccountAddressParseError;

impl fmt::Display for AccountAddressParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Unable to parse AccountAddress (must be hex string of length {})",
            AccountAddress::LENGTH
        )
    }
}
