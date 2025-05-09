// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// From https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/iota_serde.rs#L65

use super::language_storage_min::TypeTag;
use super::{AccountAddress, bigint::BigInt, language_storage_min::StructTag};
use std::{fmt, marker::PhantomData};

use std::fmt::{Debug, Write};

use serde::{
    self, Deserialize, Serialize,
    de::Deserializer,
    ser::{Error as SerError, Serializer},
};
use serde_with::{Bytes, DeserializeAs, DisplayFromStr, SerializeAs, serde_as};

/// Use with serde_as to control serde for human-readable serialization and
/// deserialization `H` : serde_as SerializeAs/DeserializeAs delegation for
/// human readable in/output `R` : serde_as SerializeAs/DeserializeAs delegation
/// for non-human readable in/output
///
/// # Example:
///
/// ```text
/// #[serde_as]
/// #[derive(Deserialize, Serialize)]
/// struct Example(#[serde_as(as = "Readable<DisplayFromStr, _>")] [u8; 20]);
/// ```
///
/// The above example will delegate human-readable serde to `DisplayFromStr`
/// and array tuple (default) for non-human-readable serializer.
pub struct Readable<H, R> {
    human_readable: PhantomData<H>,
    non_human_readable: PhantomData<R>,
}

impl<T: ?Sized, H, R> SerializeAs<T> for Readable<H, R>
where
    H: SerializeAs<T>,
    R: SerializeAs<T>,
{
    fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            H::serialize_as(value, serializer)
        } else {
            R::serialize_as(value, serializer)
        }
    }
}

impl<'de, R, H, T> DeserializeAs<'de, T> for Readable<H, R>
where
    H: DeserializeAs<'de, T>,
    R: DeserializeAs<'de, T>,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            H::deserialize_as(deserializer)
        } else {
            R::deserialize_as(deserializer)
        }
    }
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy)]
pub struct SequenceNumber(u64);

impl SerializeAs<super::types::SequenceNumber> for SequenceNumber {
    fn serialize_as<S>(value: &super::types::SequenceNumber, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = value.value().to_string();
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, super::types::SequenceNumber> for SequenceNumber {
    fn deserialize_as<D>(deserializer: D) -> Result<super::types::SequenceNumber, D::Error>
    where
        D: Deserializer<'de>,
    {
        let b = BigInt::deserialize(deserializer)?;
        Ok(super::types::SequenceNumber::from_u64(*b))
    }
}

pub struct IotaStructTag;

impl SerializeAs<StructTag> for IotaStructTag {
    fn serialize_as<S>(value: &StructTag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let f = to_iota_struct_tag_string(value).map_err(S::Error::custom)?;
        f.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, StructTag> for IotaStructTag {
    fn deserialize_as<D>(deserializer: D) -> Result<StructTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_struct_tag(&s).map_err(D::Error::custom)
    }
}

// static a1: AccountAddress = AccountAddress::from_hex_literal("0x0").unwrap();

// const IOTA_ADDRESSES: [AccountAddress; 7] = [
//     AccountAddress::from_hex_literal("0x0000000000000000000000000000000000000000000000000000000000000006").unwrap(), // AccountAddress::ZERO,
//     AccountAddress::from_hex_literal("0x1").unwrap(), // AccountAddress::ONE,
//     AccountAddress::from_hex_literal("0x2").unwrap(), // IOTA_FRAMEWORK_ADDRESS,
//     AccountAddress::from_hex_literal("0x3").unwrap(), // IOTA_SYSTEM_ADDRESS,
//     AccountAddress::from_hex_literal("0x107a").unwrap(), // STARDUST_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
//     AccountAddress::from_hex_literal("0x5").unwrap(), // IOTA_SYSTEM_STATE_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
//     AccountAddress::from_hex_literal("0x6").unwrap(), // IOTA_CLOCK_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
// ];

/// Serialize StructTag as a string, retaining the leading zeros in the address.
pub fn to_iota_struct_tag_string(value: &StructTag) -> Result<String, fmt::Error> {
    let mut f = String::new();
    let IOTA_ADDRESSES: [AccountAddress; 7] = [
        AccountAddress::from_hex_literal("0x6").unwrap(), // AccountAddress::ZERO,
        AccountAddress::from_hex_literal("0x1").unwrap(), // AccountAddress::ONE,
        AccountAddress::from_hex_literal("0x2").unwrap(), // IOTA_FRAMEWORK_ADDRESS,
        AccountAddress::from_hex_literal("0x3").unwrap(), // IOTA_SYSTEM_ADDRESS,
        AccountAddress::from_hex_literal("0x107a").unwrap(), // STARDUST_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
        AccountAddress::from_hex_literal("0x5").unwrap(), // IOTA_SYSTEM_STATE_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
        AccountAddress::from_hex_literal("0x6").unwrap(), // IOTA_CLOCK_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
    ];
    // trim leading zeros if address is in IOTA_ADDRESSES
    let address = if IOTA_ADDRESSES.contains(&value.address) {
        value.address.short_str_lossless()
    } else {
        value.address.to_canonical_string(/* with_prefix */ false)
    };

    write!(f, "0x{}::{}::{}", address, value.module, value.name)?;
    if let Some(first_ty) = value.type_params.first() {
        write!(f, "<")?;
        write!(f, "{}", to_iota_type_tag_string(first_ty)?)?;
        for ty in value.type_params.iter().skip(1) {
            write!(f, ", {}", to_iota_type_tag_string(ty)?)?;
        }
        write!(f, ">")?;
    }
    Ok(f)
}

fn to_iota_type_tag_string(value: &TypeTag) -> Result<String, fmt::Error> {
    match value {
        TypeTag::Vector(t) => Ok(format!("vector<{}>", to_iota_type_tag_string(t)?)),
        TypeTag::Struct(s) => to_iota_struct_tag_string(s),
        _ => Ok(value.to_string()),
    }
}
