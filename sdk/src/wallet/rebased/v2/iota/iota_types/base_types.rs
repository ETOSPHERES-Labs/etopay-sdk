use std::{
    cmp::max,
    convert::{TryFrom, TryInto},
    fmt::{self, format},
    str::FromStr,
};

pub use super::committee::EpochId;

use super::ObjectDigest;
use super::Readable;

use crate::wallet::rebased::v2::mowe::move_core_types::language_storage::StructTag;
use crate::wallet::rebased::v2::mowe::move_core_types::language_storage::TypeTag;

use super::HexAccountAddress;
use crate::wallet::rebased::{
    RebasedError,
    encoding::{Encoding, Hex},
    v2::AccountAddress,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

// AccountAddress::LENGTH;
pub const LENGTH: usize = 32;

pub const IOTA_ADDRESS_LENGTH: usize = LENGTH;

pub type VersionDigest = (SequenceNumber, ObjectDigest);

#[serde_as]
#[derive(Eq, Default, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize, JsonSchema)]
pub struct IotaAddress(
    #[schemars(with = "Hex")]
    #[serde_as(as = "Readable<Hex, _>")]
    [u8; IOTA_ADDRESS_LENGTH],
);

impl IotaAddress {
    pub const ZERO: Self = Self([0u8; IOTA_ADDRESS_LENGTH]);

    pub fn new(bytes: [u8; IOTA_ADDRESS_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Convert the address to a byte buffer.
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Parse a IotaAddress from a byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, RebasedError> {
        <[u8; IOTA_ADDRESS_LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| RebasedError::InvalidAddress)
            .map(IotaAddress)
    }

    /// Return the underlying byte array of a IotaAddress.
    pub fn to_inner(self) -> [u8; IOTA_ADDRESS_LENGTH] {
        self.0
    }

    /// Serialize an `Option<IotaAddress>` in Hex.
    pub fn optional_address_as_hex<S>(key: &Option<IotaAddress>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&key.map(Hex::encode).unwrap_or_default())
    }

    /// Deserialize into an `Option<IotaAddress>`.
    pub fn optional_address_from_hex<'de, D>(deserializer: D) -> Result<Option<IotaAddress>, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dbh_value = Hex::decode(&s).map_err(serde::de::Error::custom)?;
        let value = IotaAddress::from_bytes(&dbh_value).map_err(serde::de::Error::custom)?;
        // let value = decode_bytes_hex(&s).map_err(serde::de::Error::custom)?;
        Ok(Some(value))
    }
}

impl AsRef<[u8]> for IotaAddress {
    fn as_ref(&self) -> &[u8] {
        &self.0[..]
    }
}

impl TryFrom<&[u8]> for IotaAddress {
    type Error = RebasedError;

    /// Tries to convert the provided byte array into a IotaAddress.
    fn try_from(bytes: &[u8]) -> Result<Self, RebasedError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for IotaAddress {
    type Error = RebasedError;

    /// Tries to convert the provided byte buffer into a IotaAddress.
    fn try_from(bytes: Vec<u8>) -> Result<Self, RebasedError> {
        Self::from_bytes(bytes)
    }
}

impl fmt::Debug for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

impl fmt::Display for IotaAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{}", Hex::encode(self.0))
    }
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ObjectID(
    #[schemars(with = "Hex")]
    #[serde_as(as = "Readable<HexAccountAddress, _>")]
    AccountAddress,
);

impl ObjectID {
    /// The number of bytes in an address.
    pub const LENGTH: usize = AccountAddress::LENGTH;
    /// Hex address: 0x0
    pub const ZERO: Self = Self::new([0u8; Self::LENGTH]);
    pub const MAX: Self = Self::new([0xff; Self::LENGTH]);
    /// Create a new ObjectID
    pub const fn new(obj_id: [u8; Self::LENGTH]) -> Self {
        Self(AccountAddress::new(obj_id))
    }

    /// Parse the ObjectID from byte array or buffer.
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, RebasedError> {
        <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
            .map_err(|_| RebasedError::ObjectIDParseError(format!("@ObjectId -> from_bytes()")))
            .map(ObjectID::new)
    }
}

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

impl TryFrom<&[u8]> for ObjectID {
    type Error = RebasedError;

    /// Tries to convert the provided byte array into ObjectID.
    fn try_from(bytes: &[u8]) -> Result<ObjectID, RebasedError> {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<Vec<u8>> for ObjectID {
    type Error = RebasedError;

    /// Tries to convert the provided byte buffer into ObjectID.
    fn try_from(bytes: Vec<u8>) -> Result<ObjectID, RebasedError> {
        Self::from_bytes(bytes)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Default, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SequenceNumber(u64);

impl SequenceNumber {
    pub fn one_before(&self) -> Option<SequenceNumber> {
        if self.0 == 0 {
            None
        } else {
            Some(SequenceNumber(self.0 - 1))
        }
    }

    pub fn next(&self) -> SequenceNumber {
        SequenceNumber(self.0 + 1)
    }
}

impl SequenceNumber {
    pub const MIN: SequenceNumber = SequenceNumber(u64::MIN);
    pub const MAX: SequenceNumber = SequenceNumber(0x7fff_ffff_ffff_ffff);
    pub const CANCELLED_READ: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 1);
    pub const CONGESTED: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 2);
    pub const RANDOMNESS_UNAVAILABLE: SequenceNumber = SequenceNumber(SequenceNumber::MAX.value() + 3);

    pub const fn new() -> Self {
        SequenceNumber(0)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }

    pub const fn from_u64(u: u64) -> Self {
        SequenceNumber(u)
    }

    pub fn increment(&mut self) {
        assert_ne!(self.0, u64::MAX);
        self.0 += 1;
    }

    pub fn increment_to(&mut self, next: SequenceNumber) {
        debug_assert!(*self < next, "Not an increment: {} to {}", self, next);
        *self = next;
    }

    pub fn decrement(&mut self) {
        assert_ne!(self.0, 0);
        self.0 -= 1;
    }

    pub fn decrement_to(&mut self, prev: SequenceNumber) {
        debug_assert!(prev < *self, "Not a decrement: {} to {}", self, prev);
        *self = prev;
    }

    /// Returns a new sequence number that is greater than all `SequenceNumber`s
    /// in `inputs`, assuming this operation will not overflow.
    #[must_use]
    pub fn lamport_increment(inputs: impl IntoIterator<Item = SequenceNumber>) -> SequenceNumber {
        let max_input = inputs.into_iter().fold(SequenceNumber::new(), max);

        // TODO: Ensure this never overflows.
        // Option 1: Freeze the object when sequence number reaches MAX.
        // Option 2: Reject tx with MAX sequence number.
        // Issue #182.
        assert_ne!(max_input.0, u64::MAX);

        SequenceNumber(max_input.0 + 1)
    }

    pub fn is_cancelled(&self) -> bool {
        self == &SequenceNumber::CANCELLED_READ
            || self == &SequenceNumber::CONGESTED
            || self == &SequenceNumber::RANDOMNESS_UNAVAILABLE
    }

    pub fn is_valid(&self) -> bool {
        self < &SequenceNumber::MAX
    }
}

impl fmt::Display for SequenceNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}

impl From<SequenceNumber> for u64 {
    fn from(val: SequenceNumber) -> Self {
        val.0
    }
}

impl From<u64> for SequenceNumber {
    fn from(value: u64) -> Self {
        SequenceNumber(value)
    }
}

impl From<SequenceNumber> for usize {
    fn from(value: SequenceNumber) -> Self {
        value.0 as usize
    }
}

pub type ObjectRef = (ObjectID, SequenceNumber, ObjectDigest);

/// Wrapper around StructTag with a space-efficient representation for common
/// types like coins The StructTag for a gas coin is 84 bytes, so using 1 byte
/// instead is a win. The inner representation is private to prevent incorrectly
/// constructing an `Other` instead of one of the specialized variants, e.g.
/// `Other(GasCoin::type_())` instead of `GasCoin`
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MoveObjectType(MoveObjectType_);

/// Even though it is declared public, it is the "private", internal
/// representation for `MoveObjectType`
#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Deserialize, Serialize, Hash)]
pub enum MoveObjectType_ {
    /// A type that is not `0x2::coin::Coin<T>`
    Other(StructTag),
    /// An IOTA coin (i.e., `0x2::coin::Coin<0x2::iota::IOTA>`)
    GasCoin,
    /// A record of a staked IOTA coin (i.e., `0x3::staking_pool::StakedIota`)
    StakedIota,
    /// A non-IOTA coin type (i.e., `0x2::coin::Coin<T> where T !=
    /// 0x2::iota::IOTA`)
    Coin(TypeTag),
    // NOTE: if adding a new type here, and there are existing on-chain objects of that
    // type with Other(_), that is ok, but you must hand-roll PartialEq/Eq/Ord/maybe Hash
    // to make sure the new type and Other(_) are interpreted consistently.
}

impl MoveObjectType {
    /// Return true if `self` is `0x2::coin::Coin<T>` for some T (note: T can be
    /// IOTA)
    pub fn is_coin(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin | MoveObjectType_::Coin(_) => true,
            MoveObjectType_::StakedIota | MoveObjectType_::Other(_) => false,
        }
    }

    /// Return true if `self` is 0x2::coin::Coin<0x2::iota::IOTA>
    pub fn is_gas_coin(&self) -> bool {
        match &self.0 {
            MoveObjectType_::GasCoin => true,
            MoveObjectType_::StakedIota | MoveObjectType_::Coin(_) | MoveObjectType_::Other(_) => false,
        }
    }
}

// is_coin
