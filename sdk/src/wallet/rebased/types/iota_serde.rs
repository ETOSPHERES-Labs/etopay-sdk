// // Copyright (c) Mysten Labs, Inc.
// // Modifications Copyright (c) 2024 IOTA Stiftung
// // SPDX-License-Identifier: Apache-2.0

// use std::{
//     fmt,
//     fmt::{Debug, Display, Formatter, Write},
//     marker::PhantomData,
//     ops::Deref,
//     str::FromStr,
// };

// use fastcrypto::encoding::Hex;
// use iota_protocol_config::ProtocolVersion;
// use move_core_types::{
//     account_address::AccountAddress,
//     language_storage::{StructTag, TypeTag},
// };
// use serde::{
//     self, Deserialize, Serialize,
//     de::{Deserializer, Error},
//     ser::{Error as SerError, Serializer},
// };
// use serde_with::{Bytes, DeserializeAs, DisplayFromStr, SerializeAs, serde_as};

// use crate::{
//     IOTA_CLOCK_ADDRESS, IOTA_FRAMEWORK_ADDRESS, IOTA_SYSTEM_ADDRESS, IOTA_SYSTEM_STATE_ADDRESS, STARDUST_ADDRESS,
//     parse_iota_struct_tag, parse_iota_type_tag,
// };

// #[inline]
// fn to_custom_error<'de, D, E>(e: E) -> D::Error
// where
//     E: Debug,
//     D: Deserializer<'de>,
// {
//     Error::custom(format!("byte deserialization failed, cause by: {:?}", e))
// }

// #[inline]
// fn to_custom_ser_error<S, E>(e: E) -> S::Error
// where
//     E: Debug,
//     S: Serializer,
// {
//     S::Error::custom(format!("byte serialization failed, cause by: {:?}", e))
// }

// /// Use with serde_as to control serde for human-readable serialization and
// /// deserialization `H` : serde_as SerializeAs/DeserializeAs delegation for
// /// human readable in/output `R` : serde_as SerializeAs/DeserializeAs delegation
// /// for non-human readable in/output
// ///
// /// # Example:
// ///
// /// ```text
// /// #[serde_as]
// /// #[derive(Deserialize, Serialize)]
// /// struct Example(#[serde_as(as = "Readable<DisplayFromStr, _>")] [u8; 20]);
// /// ```
// ///
// /// The above example will delegate human-readable serde to `DisplayFromStr`
// /// and array tuple (default) for non-human-readable serializer.
// pub struct Readable<H, R> {
//     human_readable: PhantomData<H>,
//     non_human_readable: PhantomData<R>,
// }

// impl<T: ?Sized, H, R> SerializeAs<T> for Readable<H, R>
// where
//     H: SerializeAs<T>,
//     R: SerializeAs<T>,
// {
//     fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         if serializer.is_human_readable() {
//             H::serialize_as(value, serializer)
//         } else {
//             R::serialize_as(value, serializer)
//         }
//     }
// }

// impl<'de, R, H, T> DeserializeAs<'de, T> for Readable<H, R>
// where
//     H: DeserializeAs<'de, T>,
//     R: DeserializeAs<'de, T>,
// {
//     fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         if deserializer.is_human_readable() {
//             H::deserialize_as(deserializer)
//         } else {
//             R::deserialize_as(deserializer)
//         }
//     }
// }

// /// custom serde for AccountAddress
// pub struct HexAccountAddress;

// impl SerializeAs<AccountAddress> for HexAccountAddress {
//     fn serialize_as<S>(value: &AccountAddress, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         Hex::serialize_as(value, serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, AccountAddress> for HexAccountAddress {
//     fn deserialize_as<D>(deserializer: D) -> Result<AccountAddress, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         if s.starts_with("0x") {
//             AccountAddress::from_hex_literal(&s)
//         } else {
//             AccountAddress::from_hex(&s)
//         }
//         .map_err(to_custom_error::<'de, D, _>)
//     }
// }

// /// Serializes a bitmap according to the roaring bitmap on-disk standard.
// /// <https://github.com/RoaringBitmap/RoaringFormatSpec>
// pub struct IotaBitmap;

// impl SerializeAs<roaring::RoaringBitmap> for IotaBitmap {
//     fn serialize_as<S>(source: &roaring::RoaringBitmap, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut bytes = vec![];

//         source.serialize_into(&mut bytes).map_err(to_custom_ser_error::<S, _>)?;
//         Bytes::serialize_as(&bytes, serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, roaring::RoaringBitmap> for IotaBitmap {
//     fn deserialize_as<D>(deserializer: D) -> Result<roaring::RoaringBitmap, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let bytes: Vec<u8> = Bytes::deserialize_as(deserializer)?;
//         roaring::RoaringBitmap::deserialize_from(&bytes[..]).map_err(to_custom_error::<'de, D, _>)
//     }
// }

// pub struct IotaStructTag;

// impl SerializeAs<StructTag> for IotaStructTag {
//     fn serialize_as<S>(value: &StructTag, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let f = to_iota_struct_tag_string(value).map_err(S::Error::custom)?;
//         f.serialize(serializer)
//     }
// }

// const IOTA_ADDRESSES: [AccountAddress; 7] = [
//     AccountAddress::ZERO,
//     AccountAddress::ONE,
//     IOTA_FRAMEWORK_ADDRESS,
//     IOTA_SYSTEM_ADDRESS,
//     STARDUST_ADDRESS,
//     IOTA_SYSTEM_STATE_ADDRESS,
//     IOTA_CLOCK_ADDRESS,
// ];
// /// Serialize StructTag as a string, retaining the leading zeros in the address.
// pub fn to_iota_struct_tag_string(value: &StructTag) -> Result<String, fmt::Error> {
//     let mut f = String::new();
//     // trim leading zeros if address is in IOTA_ADDRESSES
//     let address = if IOTA_ADDRESSES.contains(&value.address) {
//         value.address.short_str_lossless()
//     } else {
//         value.address.to_canonical_string(/* with_prefix */ false)
//     };

//     write!(f, "0x{}::{}::{}", address, value.module, value.name)?;
//     if let Some(first_ty) = value.type_params.first() {
//         write!(f, "<")?;
//         write!(f, "{}", to_iota_type_tag_string(first_ty)?)?;
//         for ty in value.type_params.iter().skip(1) {
//             write!(f, ", {}", to_iota_type_tag_string(ty)?)?;
//         }
//         write!(f, ">")?;
//     }
//     Ok(f)
// }

// fn to_iota_type_tag_string(value: &TypeTag) -> Result<String, fmt::Error> {
//     match value {
//         TypeTag::Vector(t) => Ok(format!("vector<{}>", to_iota_type_tag_string(t)?)),
//         TypeTag::Struct(s) => to_iota_struct_tag_string(s),
//         _ => Ok(value.to_string()),
//     }
// }

// impl<'de> DeserializeAs<'de, StructTag> for IotaStructTag {
//     fn deserialize_as<D>(deserializer: D) -> Result<StructTag, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         parse_iota_struct_tag(&s).map_err(D::Error::custom)
//     }
// }

// pub struct IotaTypeTag;

// impl SerializeAs<TypeTag> for IotaTypeTag {
//     fn serialize_as<S>(value: &TypeTag, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = to_iota_type_tag_string(value).map_err(S::Error::custom)?;
//         s.serialize(serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, TypeTag> for IotaTypeTag {
//     fn deserialize_as<D>(deserializer: D) -> Result<TypeTag, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         parse_iota_type_tag(&s).map_err(D::Error::custom)
//     }
// }

// #[serde_as]
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, JsonSchema)]
// pub struct BigInt<T>(
//     #[schemars(with = "String")]
//     #[serde_as(as = "DisplayFromStr")]
//     T,
// )
// where
//     T: Display + FromStr,
//     <T as FromStr>::Err: Display;

// impl<T> BigInt<T>
// where
//     T: Display + FromStr,
//     <T as FromStr>::Err: Display,
// {
//     pub fn into_inner(self) -> T {
//         self.0
//     }
// }

// impl<T> SerializeAs<T> for BigInt<T>
// where
//     T: Display + FromStr + Copy,
//     <T as FromStr>::Err: Display,
// {
//     fn serialize_as<S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         BigInt(*value).serialize(serializer)
//     }
// }

// impl<'de, T> DeserializeAs<'de, T> for BigInt<T>
// where
//     T: Display + FromStr + Copy,
//     <T as FromStr>::Err: Display,
// {
//     fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         Ok(*BigInt::deserialize(deserializer)?)
//     }
// }

// impl<T> From<T> for BigInt<T>
// where
//     T: Display + FromStr,
//     <T as FromStr>::Err: Display,
// {
//     fn from(v: T) -> BigInt<T> {
//         BigInt(v)
//     }
// }

// impl<T> Deref for BigInt<T>
// where
//     T: Display + FromStr,
//     <T as FromStr>::Err: Display,
// {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<T> Display for BigInt<T>
// where
//     T: Display + FromStr,
//     <T as FromStr>::Err: Display,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// #[serde_as]
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, JsonSchema)]
// pub struct SequenceNumber(#[schemars(with = "BigInt<u64>")] u64);

// impl SerializeAs<crate::base_types::SequenceNumber> for SequenceNumber {
//     fn serialize_as<S>(value: &crate::base_types::SequenceNumber, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = value.value().to_string();
//         s.serialize(serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, crate::base_types::SequenceNumber> for SequenceNumber {
//     fn deserialize_as<D>(deserializer: D) -> Result<crate::base_types::SequenceNumber, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let b = BigInt::deserialize(deserializer)?;
//         Ok(crate::base_types::SequenceNumber::from_u64(*b))
//     }
// }

// #[serde_as]
// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Copy, JsonSchema)]
// #[serde(rename = "ProtocolVersion")]
// pub struct AsProtocolVersion(#[schemars(with = "BigInt<u64>")] u64);

// impl SerializeAs<ProtocolVersion> for AsProtocolVersion {
//     fn serialize_as<S>(value: &ProtocolVersion, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let s = value.as_u64().to_string();
//         s.serialize(serializer)
//     }
// }

// impl<'de> DeserializeAs<'de, ProtocolVersion> for AsProtocolVersion {
//     fn deserialize_as<D>(deserializer: D) -> Result<ProtocolVersion, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let b = BigInt::<u64>::deserialize(deserializer)?;
//         Ok(ProtocolVersion::from(*b))
//     }
// }

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt,
};

use serde::{
    self, Deserialize, Serialize,
    de::{Deserializer, Error},
    ser::{Error as SerError, Serializer},
};
use serde_with::{Bytes, DeserializeAs, DisplayFromStr, SerializeAs, serde_as};

use crate::{Result, wallet::rebased::bigint::BigInt};

use super::{AccountAddress, StructTag, TypeTag};

pub struct IotaTypeTag;

impl SerializeAs<TypeTag> for IotaTypeTag {
    fn serialize_as<S>(value: &TypeTag, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = to_iota_type_tag_string(value).map_err(S::Error::custom)?;
        s.serialize(serializer)
    }
}

impl<'de> DeserializeAs<'de, TypeTag> for IotaTypeTag {
    fn deserialize_as<D>(deserializer: D) -> Result<TypeTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        parse_iota_type_tag(&s).map_err(D::Error::custom)
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParsedType {
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Bool,
    Address,
    Signer,
    Vector(Box<ParsedType>),
    Struct(ParsedStructType),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedFqName {
    pub module: ParsedModuleId,
    pub name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedModuleId {
    pub address: ParsedAddress,
    pub name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedStructType {
    pub fq_name: ParsedFqName,
    pub type_args: Vec<ParsedType>,
}

// Parsed Address, either a name or a numerical address
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParsedAddress {
    Named(String),
    Numerical(NumericalAddress),
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u32)]
/// Number format enum, the u32 value represents the base
pub enum NumberFormat {
    Decimal = 10,
    Hex = 16,
}

// /// A struct that represents an account address.
// #[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
// pub struct AccountAddress([u8; AccountAddress::LENGTH]);

// impl AccountAddress {
//     pub const fn new(address: [u8; Self::LENGTH]) -> Self {
//         Self(address)
//     }

//     /// The number of bytes in an address.
//     pub const LENGTH: usize = 32;

//     pub const ZERO: Self = Self([0u8; Self::LENGTH]);

//     /// Hex address: 0x1
//     pub const ONE: Self = Self::get_hex_address_one();

//     /// Hex address: 0x2
//     pub const TWO: Self = Self::get_hex_address_two();

//     pub const fn from_suffix(suffix: u16) -> AccountAddress {
//         let mut addr = [0u8; AccountAddress::LENGTH];
//         let [hi, lo] = suffix.to_be_bytes();
//         addr[AccountAddress::LENGTH - 2] = hi;
//         addr[AccountAddress::LENGTH - 1] = lo;
//         AccountAddress::new(addr)
//     }

//     const fn get_hex_address_one() -> Self {
//         let mut addr = [0u8; AccountAddress::LENGTH];
//         addr[AccountAddress::LENGTH - 1] = 1u8;
//         Self(addr)
//     }

//     const fn get_hex_address_two() -> Self {
//         let mut addr = [0u8; AccountAddress::LENGTH];
//         addr[AccountAddress::LENGTH - 1] = 2u8;
//         Self(addr)
//     }

//     // pub fn random() -> Self {
//     //     let mut rng = OsRng;
//     //     let buf: [u8; Self::LENGTH] = rng.gen();
//     //     Self(buf)
//     // }
// }

/// Numerical address represents non-named address values
/// or the assigned value of a named address
#[derive(Clone, Copy)]
pub struct NumericalAddress {
    /// the number for the address
    bytes: AccountAddress,
    /// The format (e.g. decimal or hex) for displaying the number
    format: NumberFormat,
}

/// Raw index type used in ids. 16 bits are sufficient currently.
pub type RawIndex = u16;

/// Identifier for a module.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct ModuleId(RawIndex);

/// Identifier for a datatype, relative to module.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct DatatypeId(Symbol);

/// Representation of a symbol.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Symbol(usize);

/// Represents a type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Type {
    Primitive(PrimitiveType),
    Tuple(Vec<Type>),
    Vector(Box<Type>),
    Datatype(ModuleId, DatatypeId, Vec<Type>),
    TypeParameter(u16),

    // Types only appearing in programs.
    Reference(bool, Box<Type>),

    // Types only appearing in specifications
    Fun(Vec<Type>, Box<Type>),
    TypeDomain(Box<Type>),
    ResourceDomain(ModuleId, DatatypeId, Option<Vec<Type>>),

    // Temporary types used during type checking
    Error,
    Var(u16),
}

impl Type {
    pub fn new_prim(p: PrimitiveType) -> Type {
        Type::Primitive(p)
    }

    /// Determines whether this is a type parameter.
    pub fn is_type_parameter(&self) -> bool {
        matches!(self, Type::TypeParameter(..))
    }

    /// Determines whether this is a reference.
    pub fn is_reference(&self) -> bool {
        matches!(self, Type::Reference(_, _))
    }

    /// Determines whether this is a mutable reference.
    pub fn is_mutable_reference(&self) -> bool {
        matches!(self, Type::Reference(true, _))
    }

    /// Determines whether this is an immutable reference.
    pub fn is_immutable_reference(&self) -> bool {
        matches!(self, Type::Reference(false, _))
    }

    /// Determines whether this type is a struct.
    pub fn is_struct(&self) -> bool {
        matches!(self, Type::Datatype(..))
    }

    /// Determines whether this type is a vector
    pub fn is_vector(&self) -> bool {
        matches!(self, Type::Vector(..))
    }

    /// Determines whether this is a struct, or a vector of structs, or a
    /// reference to any of those.
    pub fn is_struct_or_vector_of_struct(&self) -> bool {
        match self.skip_reference() {
            Type::Datatype(..) => true,
            Type::Vector(ety) => ety.is_struct_or_vector_of_struct(),
            _ => false,
        }
    }

    /// Returns true if this type is a specification language only type or
    /// contains specification language only types
    pub fn is_spec(&self) -> bool {
        use Type::*;
        match self {
            Primitive(p) => p.is_spec(),
            Fun(..) | TypeDomain(..) | ResourceDomain(..) | Error => true,
            Var(..) | TypeParameter(..) => false,
            Tuple(ts) => ts.iter().any(|t| t.is_spec()),
            Datatype(_, _, ts) => ts.iter().any(|t| t.is_spec()),
            Vector(et) => et.is_spec(),
            Reference(_, bt) => bt.is_spec(),
        }
    }

    /// Returns true if this is a bool.
    pub fn is_bool(&self) -> bool {
        if let Type::Primitive(PrimitiveType::Bool) = self {
            return true;
        }
        false
    }

    /// Returns true if this is any number type.
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            Type::Primitive(
                PrimitiveType::U8
                    | PrimitiveType::U16
                    | PrimitiveType::U32
                    | PrimitiveType::U64
                    | PrimitiveType::U128
                    | PrimitiveType::U256
                    | PrimitiveType::Num,
            )
        )
    }
    /// Returns true if this is an address or signer type.
    pub fn is_signer_or_address(&self) -> bool {
        matches!(
            self,
            Type::Primitive(PrimitiveType::Signer) | Type::Primitive(PrimitiveType::Address)
        )
    }

    /// Return true if this is an account address
    pub fn is_address(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Address))
    }

    /// Return true if this is an account address
    pub fn is_signer(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Signer))
    }

    /// Test whether this type can be used to substitute a type parameter
    pub fn can_be_type_argument(&self) -> bool {
        match self {
            Type::Primitive(p) => !p.is_spec(),
            Type::Tuple(..) => false,
            Type::Vector(e) => e.can_be_type_argument(),
            Type::Datatype(_, _, insts) => insts.iter().all(|e| e.can_be_type_argument()),
            Type::TypeParameter(..) => true,
            // references cannot be a type argument
            Type::Reference(..) => false,
            // spec types cannot be a type argument
            Type::Fun(..) | Type::TypeDomain(..) | Type::ResourceDomain(..) | Type::Var(..) | Type::Error => false,
        }
    }

    /// Skip reference type.
    pub fn skip_reference(&self) -> &Type {
        if let Type::Reference(_, bt) = self { bt } else { self }
    }

    /// If this is a datatype, replace the type instantiation.
    pub fn replace_datatype_instantiation(&self, inst: &[Type]) -> Type {
        match self {
            Type::Datatype(mid, sid, _) => Type::Datatype(*mid, *sid, inst.to_vec()),
            _ => self.clone(),
        }
    }

    /// Require this to be a datatype, if so extracts its content.
    pub fn require_datatype(&self) -> (ModuleId, DatatypeId, &[Type]) {
        if let Type::Datatype(mid, sid, targs) = self {
            (*mid, *sid, targs.as_slice())
        } else {
            panic!("expected `Type::Struct`, found: `{:?}`", self)
        }
    }

    /// Instantiates type parameters in this type.
    pub fn instantiate(&self, params: &[Type]) -> Type {
        if params.is_empty() {
            self.clone()
        } else {
            self.replace(Some(params), None)
        }
    }

    /// Instantiate type parameters in the vector of types.
    pub fn instantiate_vec(vec: Vec<Type>, params: &[Type]) -> Vec<Type> {
        if params.is_empty() {
            vec
        } else {
            vec.into_iter().map(|ty| ty.instantiate(params)).collect()
        }
    }

    /// Instantiate type parameters in the slice of types.
    pub fn instantiate_slice(slice: &[Type], params: &[Type]) -> Vec<Type> {
        if params.is_empty() {
            slice.to_owned()
        } else {
            slice.iter().map(|ty| ty.instantiate(params)).collect()
        }
    }

    /// Convert a partial assignment for type parameters into an instantiation.
    pub fn type_param_map_to_inst(arity: usize, map: BTreeMap<u16, Type>) -> Vec<Type> {
        let mut inst: Vec<_> = (0..arity).map(|i| Type::TypeParameter(i as u16)).collect();
        for (idx, ty) in map {
            inst[idx as usize] = ty;
        }
        inst
    }

    /// A helper function to do replacement of type parameters.
    fn replace(&self, params: Option<&[Type]>, subs: Option<&Substitution>) -> Type {
        let replace_vec = |types: &[Type]| types.iter().map(|t| t.replace(params, subs)).collect();
        match self {
            Type::TypeParameter(i) => {
                if let Some(ps) = params {
                    ps[*i as usize].clone()
                } else {
                    self.clone()
                }
            }
            Type::Var(i) => {
                if let Some(s) = subs {
                    if let Some(t) = s.subs.get(i) {
                        // Recursively call replacement again here, in case the substitution s
                        // refers to type variables.
                        // TODO: a more efficient approach is to maintain that type assignments
                        // are always fully specialized w.r.t. to the substitution.
                        t.replace(params, subs)
                    } else {
                        self.clone()
                    }
                } else {
                    self.clone()
                }
            }
            Type::Reference(is_mut, bt) => Type::Reference(*is_mut, Box::new(bt.replace(params, subs))),
            Type::Datatype(mid, sid, args) => Type::Datatype(*mid, *sid, replace_vec(args)),
            Type::Fun(args, result) => Type::Fun(replace_vec(args), Box::new(result.replace(params, subs))),
            Type::Tuple(args) => Type::Tuple(replace_vec(args)),
            Type::Vector(et) => Type::Vector(Box::new(et.replace(params, subs))),
            Type::TypeDomain(et) => Type::TypeDomain(Box::new(et.replace(params, subs))),
            Type::ResourceDomain(mid, sid, args_opt) => {
                Type::ResourceDomain(*mid, *sid, args_opt.as_ref().map(|args| replace_vec(args)))
            }
            Type::Primitive(..) | Type::Error => self.clone(),
        }
    }

    /// Checks whether this type contains a type for which the predicate is
    /// true.
    pub fn contains<P>(&self, p: &P) -> bool
    where
        P: Fn(&Type) -> bool,
    {
        if p(self) {
            true
        } else {
            let contains_vec = |ts: &[Type]| ts.iter().any(p);
            match self {
                Type::Reference(_, bt) => bt.contains(p),
                Type::Datatype(_, _, args) => contains_vec(args),
                Type::Fun(args, result) => contains_vec(args) || result.contains(p),
                Type::Tuple(args) => contains_vec(args),
                Type::Vector(et) => et.contains(p),
                _ => false,
            }
        }
    }

    /// Returns true if this type is incomplete, i.e. contains any type
    /// variables.
    pub fn is_incomplete(&self) -> bool {
        use Type::*;
        match self {
            Var(_) => true,
            Tuple(ts) => ts.iter().any(|t| t.is_incomplete()),
            Fun(ts, r) => ts.iter().any(|t| t.is_incomplete()) || r.is_incomplete(),
            Datatype(_, _, ts) => ts.iter().any(|t| t.is_incomplete()),
            Vector(et) => et.is_incomplete(),
            Reference(_, bt) => bt.is_incomplete(),
            TypeDomain(bt) => bt.is_incomplete(),
            Error | Primitive(..) | TypeParameter(_) | ResourceDomain(..) => false,
        }
    }

    /// Return true if this type contains generic types (i.e., types that can be
    /// instantiated).
    pub fn is_open(&self) -> bool {
        let mut has_var = false;
        self.visit(&mut |t| has_var = has_var || matches!(t, Type::TypeParameter(_)));
        has_var
    }

    /// Compute used modules in this type, adding them to the passed set.
    pub fn module_usage(&self, usage: &mut BTreeSet<ModuleId>) {
        use Type::*;
        match self {
            Tuple(ts) => ts.iter().for_each(|t| t.module_usage(usage)),
            Fun(ts, r) => {
                ts.iter().for_each(|t| t.module_usage(usage));
                r.module_usage(usage);
            }
            Datatype(mid, _, ts) => {
                usage.insert(*mid);
                ts.iter().for_each(|t| t.module_usage(usage));
            }
            Vector(et) => et.module_usage(usage),
            Reference(_, bt) => bt.module_usage(usage),
            TypeDomain(bt) => bt.module_usage(usage),
            _ => {}
        }
    }

    // /// Attempt to convert this type into a normalized::Type
    // pub fn into_datatype_ty(self, env: &GlobalEnv) -> Option<MType> {
    //     use Type::*;
    //     match self {
    //         Datatype(mid, sid, ts) => env.get_datatype(mid, sid, &ts),
    //         _ => None,
    //     }
    // }

    // /// Attempt to convert this type into a normalized::Type
    // pub fn into_normalized_type(self, env: &GlobalEnv) -> Option<MType> {
    //     use Type::*;
    //     match self {
    //         Primitive(p) => Some(
    //             p.into_normalized_type()
    //                 .expect("Invariant violation: unexpected spec primitive"),
    //         ),
    //         Datatype(mid, sid, ts) => env.get_datatype(mid, sid, &ts),
    //         Vector(et) => Some(MType::Vector(Box::new(et.into_normalized_type(env).expect(
    //             "Invariant violation: vector type argument contains incomplete, tuple, or spec type",
    //         )))),
    //         Reference(r, t) => {
    //             if r {
    //                 Some(MType::MutableReference(Box::new(t.into_normalized_type(env).expect(
    //                     "Invariant violation: reference type contains incomplete, tuple, or spec type",
    //                 ))))
    //             } else {
    //                 Some(MType::Reference(Box::new(t.into_normalized_type(env).expect(
    //                     "Invariant violation: reference type contains incomplete, tuple, or spec type",
    //                 ))))
    //             }
    //         }
    //         TypeParameter(idx) => Some(MType::TypeParameter(idx)),
    //         Tuple(..) | Error | Fun(..) | TypeDomain(..) | ResourceDomain(..) | Var(..) => None,
    //     }
    // }

    // /// Attempt to convert this type into a language_storage::StructTag
    // pub fn into_struct_tag(self, env: &GlobalEnv) -> Option<StructTag> {
    //     self.into_datatype_ty(env)?.into_struct_tag()
    // }

    // /// Attempt to convert this type into a language_storage::TypeTag
    // pub fn into_type_tag(self, env: &GlobalEnv) -> Option<TypeTag> {
    //     self.into_normalized_type(env)?.into_type_tag()
    // }

    // /// Create a `Type` from `t`
    // pub fn from_type_tag(t: &TypeTag, env: &GlobalEnv) -> Self {
    //     use Type::*;
    //     match t {
    //         TypeTag::Bool => Primitive(PrimitiveType::Bool),
    //         TypeTag::U8 => Primitive(PrimitiveType::U8),
    //         TypeTag::U16 => Primitive(PrimitiveType::U8),
    //         TypeTag::U32 => Primitive(PrimitiveType::U8),
    //         TypeTag::U64 => Primitive(PrimitiveType::U64),
    //         TypeTag::U128 => Primitive(PrimitiveType::U128),
    //         TypeTag::U256 => Primitive(PrimitiveType::U8),
    //         TypeTag::Address => Primitive(PrimitiveType::Address),
    //         TypeTag::Signer => Primitive(PrimitiveType::Signer),
    //         TypeTag::Struct(s) => {
    //             let qid = env
    //                 .find_datatype_by_tag(s)
    //                 .unwrap_or_else(|| panic!("Invariant violation: couldn't resolve datatype {:?}", s));
    //             let type_args = s.type_params.iter().map(|arg| Self::from_type_tag(arg, env)).collect();
    //             Datatype(qid.module_id, qid.id, type_args)
    //         }
    //         TypeTag::Vector(type_param) => Vector(Box::new(Self::from_type_tag(type_param, env))),
    //     }
    // }

    /// Get the unbound type variables in the type.
    pub fn get_vars(&self) -> BTreeSet<u16> {
        let mut vars = BTreeSet::new();
        self.internal_get_vars(&mut vars);
        vars
    }

    fn internal_get_vars(&self, vars: &mut BTreeSet<u16>) {
        use Type::*;
        match self {
            Var(id) => {
                vars.insert(*id);
            }
            Tuple(ts) => ts.iter().for_each(|t| t.internal_get_vars(vars)),
            Fun(ts, r) => {
                r.internal_get_vars(vars);
                ts.iter().for_each(|t| t.internal_get_vars(vars));
            }
            Datatype(_, _, ts) => ts.iter().for_each(|t| t.internal_get_vars(vars)),
            Vector(et) => et.internal_get_vars(vars),
            Reference(_, bt) => bt.internal_get_vars(vars),
            TypeDomain(bt) => bt.internal_get_vars(vars),
            Error | Primitive(..) | TypeParameter(..) | ResourceDomain(..) => {}
        }
    }

    pub fn visit<F: FnMut(&Type)>(&self, visitor: &mut F) {
        let visit_slice = |s: &[Type], visitor: &mut F| {
            for ty in s {
                ty.visit(visitor);
            }
        };
        match self {
            Type::Tuple(tys) => visit_slice(tys, visitor),
            Type::Vector(bt) => bt.visit(visitor),
            Type::Datatype(_, _, tys) => visit_slice(tys, visitor),
            Type::Reference(_, ty) => ty.visit(visitor),
            Type::Fun(tys, ty) => {
                visit_slice(tys, visitor);
                ty.visit(visitor);
            }
            Type::TypeDomain(bt) => bt.visit(visitor),
            _ => {}
        }
        visitor(self)
    }
}

/// A type substitution.
#[derive(Debug, Clone)]
pub struct Substitution {
    subs: BTreeMap<u16, Type>,
}

impl Substitution {
    /// Creates a new substitution.
    pub fn new() -> Self {
        Self { subs: BTreeMap::new() }
    }
}

/// Represents a primitive (builtin) type.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum PrimitiveType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Address,
    Signer,
    // Types only appearing in specifications
    Num,
    Range,
    EventStore,
}

/// Parse `s` as a type: Either a struct type (see `parse_iota_struct_tag`), a
/// primitive type, or a vector with a type parameter. Parsing succeeds if and
/// only if `s` matches this format exactly, with no remaining input. This
/// function is intended for use within the authority codebase.
pub fn parse_iota_type_tag(s: &str) -> Result<TypeTag> {
    use ParsedType;
    ParsedType::parse(s)?.into_type_tag(&resolve_address)
}

/// Resolve well-known named addresses into numeric addresses.
pub fn resolve_address(addr: &str) -> Option<AccountAddress> {
    match addr {
        "std" => Some(MOVE_STDLIB_ADDRESS),
        "iota" => Some(IOTA_FRAMEWORK_ADDRESS),
        "iota_system" => Some(IOTA_SYSTEM_ADDRESS),
        "stardust" => Some(AccountAddress::from_hex_literal("0x107a").unwrap()), // STARDUST_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
        "bridge" => Some(AccountAddress::from_hex_literal("0xb").unwrap()), // BRIDGE_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
        _ => None,
    }
}

impl ParsedType {
    pub fn parse(s: &str) -> Result<ParsedType> {
        parse(s, |parser| parser.parse_type())
    }
}

pub(crate) fn parse<'a, Tok: Token, R>(
    s: &'a str,
    f: impl FnOnce(&mut Parser<'a, Tok, std::vec::IntoIter<(Tok, &'a str)>>) -> Result<R>,
) -> Result<R> {
    let tokens: Vec<_> = Tok::tokenize(s)?
        .into_iter()
        .filter(|(tok, _)| !tok.is_whitespace())
        .collect();
    let mut parser = Parser::new(tokens);
    let res = f(&mut parser)?;
    if let Ok((_, contents)) = parser.advance_any() {
        bail!("Expected end of token stream. Got: {}", contents)
    }
    Ok(res)
}

fn to_iota_type_tag_string(value: &TypeTag) -> Result<String, fmt::Error> {
    match value {
        TypeTag::Vector(t) => Ok(format!("vector<{}>", to_iota_type_tag_string(t)?)),
        TypeTag::Struct(s) => to_iota_struct_tag_string(s),
        _ => Ok(value.to_string()),
    }
}

// from: sdk/typescript/utils/contants.ts
static IOTA_DECIMALS: i32 = 9;
static NANOS_PER_IOTA: BigInt<u32> = BigInt::from(1000000000);

static MOVE_STDLIB_ADDRESS: &str = "0x1";
static IOTA_FRAMEWORK_ADDRESS: &str = "0x2";
static IOTA_SYSTEM_ADDRESS: &str = "0x3";
static IOTA_CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006"; // normalizeIotaObjectId('0x6');
static IOTA_SYSTEM_MODULE_NAME: &str = "iota_system";
static IOTA_TYPE_ARG: &str = "0x2::iota::IOTA";
static IOTA_SYSTEM_STATE_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000005"; // normalizeIotaObjectId('0x5');

/*
    function normalizeIotaAddress(
        value,
        forceAdd0x,
        validate,
    ) {
        let address = value.toLowerCase().replace(/ /g, '');
        if (!forceAdd0x && address.startsWith('0x')) {
            address = address.slice(2);
        }
        address = `0x${address.padStart(32 * 2, '0')}`;
        if (validate) {
            throw new Error(`Invalid IOTA address: ${value}`);
        } else {
            return address;
        }
    }

    // normalizeIotaAddress('0x6', false, false)
    // => "0x0000000000000000000000000000000000000000000000000000000000000006"
*/

//let zero = AccountAddress::from_hex("0x0");

const IOTA_ADDRESSES: [AccountAddress; 7] = [
    AccountAddress::from_hex_literal("0x0").unwrap(), // AccountAddress::ZERO,
    AccountAddress::from_hex_literal("0x1").unwrap(), // AccountAddress::ONE,
    AccountAddress::from_hex_literal("0x2").unwrap(), // IOTA_FRAMEWORK_ADDRESS,
    AccountAddress::from_hex_literal("0x3").unwrap(), // IOTA_SYSTEM_ADDRESS,
    AccountAddress::from_hex_literal("0x107a").unwrap(), // STARDUST_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
    AccountAddress::from_hex_literal("0x5").unwrap(), // IOTA_SYSTEM_STATE_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
    AccountAddress::from_hex_literal("0x6").unwrap(), // IOTA_CLOCK_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
];

/// Serialize StructTag as a string, retaining the leading zeros in the address.
pub fn to_iota_struct_tag_string(value: &StructTag) -> Result<String, fmt::Error> {
    let mut f = String::new();
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
