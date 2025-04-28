// Copyright (c) 2022, Mysten Labs, Inc.
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

//! Encodings of binary data such as Base64 and Hex.

use base64ct::Encoding as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

use super::{RebasedError, traits::ToFromBytes};

/// Trait representing a general binary-to-string encoding.
pub trait Encoding {
    /// Decode this encoding into bytes.
    fn decode(s: &str) -> Result<Vec<u8>, RebasedError>;

    /// Encode bytes into a string.
    fn encode<T: AsRef<[u8]>>(data: T) -> String;
}

/// Cryptographic material with an immediate conversion to/from Base64 strings.
///
/// This is an [extension trait](https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html) of `ToFromBytes` above.
///
pub trait EncodeDecodeBase64: Sized {
    fn encode_base64(&self) -> String;
    fn decode_base64(value: &str) -> Result<Self, RebasedError>;
}

impl<T: ToFromBytes> EncodeDecodeBase64 for T {
    fn encode_base64(&self) -> String {
        Base64::encode(self.as_bytes())
    }

    fn decode_base64(value: &str) -> Result<Self, RebasedError> {
        let bytes = Base64::decode(value)?;
        <T as ToFromBytes>::from_bytes(&bytes)
    }
}

/// Implement `DeserializeAs<Vec<u8>>`, `DeserializeAs<[u8; N]>` and `SerializeAs<T: AsRef<[u8]>`
/// for a type that implements `Encoding`.
macro_rules! impl_serde_as_for_encoding {
    ($encoding:ty) => {
        impl<'de> DeserializeAs<'de, Vec<u8>> for $encoding {
            fn deserialize_as<D>(deserializer: D) -> Result<Vec<u8>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                Self::decode(&s).map_err(|_| serde::de::Error::custom("Deserialization failed"))
            }
        }

        impl<T> SerializeAs<T> for $encoding
        where
            T: AsRef<[u8]>,
        {
            fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                let encoded_string = Self::encode(value);
                Self(encoded_string).serialize(serializer)
            }
        }

        impl<'de, const N: usize> DeserializeAs<'de, [u8; N]> for $encoding {
            fn deserialize_as<D>(deserializer: D) -> Result<[u8; N], D::Error>
            where
                D: Deserializer<'de>,
            {
                let value: Vec<u8> = <$encoding>::deserialize_as(deserializer)?;
                value
                    .try_into()
                    .map_err(|_| serde::de::Error::custom(format!("Invalid array length, expecting {}", N)))
            }
        }
    };
}

/// Implement `TryFrom<String>` for a type that implements `Encoding`.
macro_rules! impl_try_from_string {
    ($encoding:ty) => {
        impl TryFrom<String> for $encoding {
            type Error = RebasedError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                // Error on invalid encoding
                <$encoding>::decode(&value)?;
                Ok(Self(value))
            }
        }
    };
}

/// Base64 encoding
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct Base64(String);

impl Base64 {
    // /// Decodes this Base64 encoding to bytes.
    // pub fn to_vec(&self) -> FastCryptoResult<Vec<u8>> {
    //     Self::decode(&self.0)
    // }
    /// Encodes bytes as a Base64.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(Self::encode(bytes))
    }
}

impl_serde_as_for_encoding!(Base64);
impl_try_from_string!(Base64);

impl Encoding for Base64 {
    fn decode(s: &str) -> Result<Vec<u8>, RebasedError> {
        Ok(base64ct::Base64::decode_vec(s)?)
    }

    fn encode<T: AsRef<[u8]>>(data: T) -> String {
        base64ct::Base64::encode_string(data.as_ref())
    }
}

/// Hex string encoding.
#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(try_from = "String")]
pub struct Hex(String);

impl Hex {
    // /// Create a hex encoding from a string.
    // #[cfg(test)]
    // pub fn from_string(s: &str) -> Self {
    //     Hex(s.to_string())
    // }
    // /// Decodes this hex encoding to bytes.
    // pub fn to_vec(&self) -> FastCryptoResult<Vec<u8>> {
    //     Self::decode(&self.0)
    // }
    // /// Encodes bytes as a hex string.
    // pub fn from_bytes(bytes: &[u8]) -> Self {
    //     Self(Self::encode(bytes))
    // }
    // /// Encode bytes as a hex string with a "0x" prefix.
    // pub fn encode_with_format<T: AsRef<[u8]>>(bytes: T) -> String {
    //     Self::format(&Self::encode(bytes))
    // }

    /// Get a string representation of this Hex encoding with a "0x" prefix.
    pub fn encoded_with_format(&self) -> String {
        Self::format(&self.0)
    }
    /// Add "0x" prefix to a hex string.
    fn format(hex_string: &str) -> String {
        format!("0x{}", hex_string)
    }
}

impl TryFrom<String> for Hex {
    type Error = RebasedError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let s = value.strip_prefix("0x").unwrap_or(&value);
        Ok(Self(s.to_string()))
    }
}

impl Serialize for Hex {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Hex strings are serialized with a 0x prefix which differs from the output of `Hex::encode`.
        String::serialize(&self.encoded_with_format(), serializer)
    }
}

impl_serde_as_for_encoding!(Hex);

impl Encoding for Hex {
    /// Decodes a hex string to bytes. Both upper and lower case characters are accepted, and the
    /// string may have a "0x" prefix or not.
    fn decode(s: &str) -> Result<Vec<u8>, RebasedError> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        Ok(hex::decode(s)?)
    }

    /// Hex encoding is without "0x" prefix. See `Hex::encode_with_format` for encoding with "0x".
    fn encode<T: AsRef<[u8]>>(data: T) -> String {
        hex::encode(data.as_ref())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(try_from = "String")]
pub struct Base58(String);

impl_serde_as_for_encoding!(Base58);
impl_try_from_string!(Base58);

impl Encoding for Base58 {
    fn decode(s: &str) -> Result<Vec<u8>, RebasedError> {
        Ok(bs58::decode(s).into_vec()?)
    }

    fn encode<T: AsRef<[u8]>>(data: T) -> String {
        bs58::encode(data).into_string()
    }
}
