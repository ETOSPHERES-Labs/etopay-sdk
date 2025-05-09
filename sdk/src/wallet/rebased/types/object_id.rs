// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use super::{
    super::encoding::{Encoding, Hex},
    IotaAddress,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{super::serde::Readable, AccountAddress, HexAccountAddress, ObjectDigest, SequenceNumber};

#[serde_as]
#[derive(Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectID(#[serde_as(as = "Readable<HexAccountAddress, _>")] AccountAddress);

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

pub type ObjectRef = (ObjectID, SequenceNumber, ObjectDigest);

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

    /// Const fn variant of `<ObjectID as From<AccountAddress>>::from`
    pub const fn from_address(addr: AccountAddress) -> Self {
        Self(addr)
    }

    // /// Parse the ObjectID from byte array or buffer.
    // pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<Self, ObjectIDParseError> {
    //     <[u8; Self::LENGTH]>::try_from(bytes.as_ref())
    //         .map_err(|_| ObjectIDParseError::TryFromSlice)
    //         .map(ObjectID::new)
    // }

    // /// Return the underlying bytes array of the ObjectID.
    // pub fn into_bytes(self) -> [u8; Self::LENGTH] {
    //     self.0.into_bytes()
    // }
}

impl From<AccountAddress> for ObjectID {
    fn from(address: AccountAddress) -> Self {
        Self(address)
    }
}

impl From<IotaAddress> for ObjectID {
    fn from(address: IotaAddress) -> ObjectID {
        let tmp: AccountAddress = address.into();
        tmp.into()
    }
}

impl From<IotaAddress> for AccountAddress {
    fn from(address: IotaAddress) -> Self {
        Self::new(address.0)
    }
}

impl From<ObjectID> for AccountAddress {
    fn from(obj_id: ObjectID) -> Self {
        obj_id.0
    }
}
