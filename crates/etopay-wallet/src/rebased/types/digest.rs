// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

use super::super::encoding::{Base58, Encoding};
use super::super::{RebasedError, serde::Readable};

/// A representation of a 32 byte digest
#[serde_as]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Digest(#[serde_as(as = "Readable<Base58, Bytes>")] [u8; 32]);

impl Digest {
    pub const ZERO: Self = Digest([0; 32]);

    pub const fn new(digest: [u8; 32]) -> Self {
        Self(digest)
    }

    pub fn generate<R: rand::RngCore + rand::CryptoRng>(mut rng: R) -> Self {
        let mut bytes = [0; 32];
        rng.fill_bytes(&mut bytes);
        Self(bytes)
    }

    pub fn random() -> Self {
        Self::generate(rand::rng())
    }

    pub const fn inner(&self) -> &[u8; 32] {
        &self.0
    }

    pub const fn into_inner(self) -> [u8; 32] {
        self.0
    }

    pub fn next_lexicographical(&self) -> Option<Self> {
        let mut next_digest = *self;
        let pos = next_digest.0.iter().rposition(|&byte| byte != 255)?;
        next_digest.0[pos] += 1;
        next_digest.0.iter_mut().skip(pos + 1).for_each(|byte| *byte = 0);
        Some(next_digest)
    }
}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 32]> for Digest {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<Digest> for [u8; 32] {
    fn from(digest: Digest) -> Self {
        digest.into_inner()
    }
}

impl From<[u8; 32]> for Digest {
    fn from(digest: [u8; 32]) -> Self {
        Self::new(digest)
    }
}

// impl TryFrom<Vec<u8>> for Digest {
//     type Error = IotaError;
//
//     fn try_from(bytes: Vec<u8>) -> Result<Self, IotaError> {
//         let bytes: [u8; 32] = <[u8; 32]>::try_from(&bytes[..]).map_err(|_| IotaError::InvalidDigestLength {
//             expected: 32,
//             actual: bytes.len(),
//         })?;
//
//         Ok(Self::from(bytes))
//     }
// }

impl fmt::Display for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO avoid the allocation
        f.write_str(&Base58::encode(self.0))
    }
}

impl fmt::Debug for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::LowerHex for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in self.0 {
            write!(f, "{:02X}", byte)?;
        }

        Ok(())
    }
}

/// A transaction will have a (unique) digest.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct TransactionDigest(Digest);

impl Default for TransactionDigest {
    fn default() -> Self {
        Self::ZERO
    }
}

impl TransactionDigest {
    pub const ZERO: Self = Self(Digest::ZERO);

    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }

    /// A digest we use to signify the parent transaction was the genesis,
    /// ie. for an object there is no parent digest.
    /// Note that this is not the same as the digest of the genesis transaction,
    /// which cannot be known ahead of time.
    // TODO(https://github.com/iotaledger/iota/issues/65): we can pick anything here
    pub const fn genesis_marker() -> Self {
        Self::ZERO
    }

    pub fn generate<R: rand::RngCore + rand::CryptoRng>(rng: R) -> Self {
        Self(Digest::generate(rng))
    }

    pub fn random() -> Self {
        Self(Digest::random())
    }

    pub fn inner(&self) -> &[u8; 32] {
        self.0.inner()
    }

    pub fn into_inner(self) -> [u8; 32] {
        self.0.into_inner()
    }

    pub fn base58_encode(&self) -> String {
        Base58::encode(self.0)
    }

    pub fn next_lexicographical(&self) -> Option<Self> {
        self.0.next_lexicographical().map(Self)
    }
}

impl AsRef<[u8]> for TransactionDigest {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for TransactionDigest {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl From<TransactionDigest> for [u8; 32] {
    fn from(digest: TransactionDigest) -> Self {
        digest.into_inner()
    }
}

impl From<[u8; 32]> for TransactionDigest {
    fn from(digest: [u8; 32]) -> Self {
        Self::new(digest)
    }
}

impl fmt::Display for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TransactionDigest").field(&self.0).finish()
    }
}

impl fmt::LowerHex for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

// impl TryFrom<&[u8]> for TransactionDigest {
//     type Error = crate::error::IotaError;
//
//     fn try_from(bytes: &[u8]) -> Result<Self, crate::error::IotaError> {
//         let arr: [u8; 32] = bytes
//             .try_into()
//             .map_err(|_| crate::error::IotaError::InvalidTransactionDigest)?;
//         Ok(Self::new(arr))
//     }
// }

impl std::str::FromStr for TransactionDigest {
    type Err = RebasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [0; 32];
        let buffer = Base58::decode(s)?;
        if buffer.len() != 32 {
            return Err(RebasedError::InvalidDigestLength);
        }
        result.copy_from_slice(&buffer);
        Ok(TransactionDigest::new(result))
    }
}

// Each object has a unique digest
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectDigest(Digest);

impl fmt::Debug for ObjectDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "o#{}", self.0)
    }
}

/// Representation of a Checkpoint's digest
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CheckpointDigest(Digest);

impl CheckpointDigest {
    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }

    pub fn generate<R: rand::RngCore + rand::CryptoRng>(rng: R) -> Self {
        Self(Digest::generate(rng))
    }

    pub fn random() -> Self {
        Self(Digest::random())
    }

    pub const fn inner(&self) -> &[u8; 32] {
        self.0.inner()
    }

    pub const fn into_inner(self) -> [u8; 32] {
        self.0.into_inner()
    }

    pub fn base58_encode(&self) -> String {
        Base58::encode(self.0)
    }

    pub fn next_lexicographical(&self) -> Option<Self> {
        self.0.next_lexicographical().map(Self)
    }
}

impl AsRef<[u8]> for CheckpointDigest {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl AsRef<[u8; 32]> for CheckpointDigest {
    fn as_ref(&self) -> &[u8; 32] {
        self.0.as_ref()
    }
}

impl From<CheckpointDigest> for [u8; 32] {
    fn from(digest: CheckpointDigest) -> Self {
        digest.into_inner()
    }
}

impl From<[u8; 32]> for CheckpointDigest {
    fn from(digest: [u8; 32]) -> Self {
        Self::new(digest)
    }
}

// impl TryFrom<Vec<u8>> for CheckpointDigest {
//     type Err = RebasedError;
//
//     fn try_from(bytes: Vec<u8>) -> Result<Self, Self::Err> {
//         Digest::try_from(bytes)?
//     }
// }

impl fmt::Display for CheckpointDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for CheckpointDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("CheckpointDigest").field(&self.0).finish()
    }
}

impl fmt::LowerHex for CheckpointDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for CheckpointDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl std::str::FromStr for CheckpointDigest {
    type Err = RebasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [0; 32];
        let buffer = Base58::decode(s)?;
        if buffer.len() != 32 {
            return Err(RebasedError::InvalidDigestLength);
        }
        result.copy_from_slice(&buffer);
        Ok(CheckpointDigest::new(result))
    }
}
