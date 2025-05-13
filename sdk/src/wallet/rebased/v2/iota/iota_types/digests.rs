use std::fmt;

use super::Readable;
use crate::wallet::rebased::{
    RebasedError,
    encoding::{Base58, Encoding},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};

#[serde_as]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EffectsAuxDataDigest(Digest);

impl fmt::Debug for EffectsAuxDataDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EffectsAuxDataDigest").field(&self.0).finish()
    }
}

/// A representation of a 32 byte digest
#[serde_as]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Digest(
    #[schemars(with = "Base58")]
    #[serde_as(as = "Readable<Base58, Bytes>")]
    [u8; 32],
);

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
        Self::generate(rand::thread_rng())
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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
pub struct ObjectDigest(Digest);

impl fmt::Debug for ObjectDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "o#{}", self.0)
    }
}

impl fmt::Display for ObjectDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ObjectDigest {
    pub const MIN: ObjectDigest = Self::new([u8::MIN; 32]);
    pub const MAX: ObjectDigest = Self::new([u8::MAX; 32]);
    pub const OBJECT_DIGEST_DELETED_BYTE_VAL: u8 = 99;
    pub const OBJECT_DIGEST_WRAPPED_BYTE_VAL: u8 = 88;
    pub const OBJECT_DIGEST_CANCELLED_BYTE_VAL: u8 = 77;

    /// A marker that signifies the object is deleted.
    pub const OBJECT_DIGEST_DELETED: ObjectDigest = Self::new([Self::OBJECT_DIGEST_DELETED_BYTE_VAL; 32]);

    /// A marker that signifies the object is wrapped into another object.
    pub const OBJECT_DIGEST_WRAPPED: ObjectDigest = Self::new([Self::OBJECT_DIGEST_WRAPPED_BYTE_VAL; 32]);

    pub const OBJECT_DIGEST_CANCELLED: ObjectDigest = Self::new([Self::OBJECT_DIGEST_CANCELLED_BYTE_VAL; 32]);

    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }
}

/// A transaction will have a (unique) digest.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TransactionDigest(Digest);

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

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Debug for TransactionDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TransactionDigest").field(&self.0).finish()
    }
}

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

#[serde_as]
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Serialize, Deserialize, JsonSchema)]
pub struct TransactionEventsDigest(Digest);

impl TransactionEventsDigest {
    pub const ZERO: Self = Self(Digest::ZERO);

    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }

    pub fn random() -> Self {
        Self(Digest::random())
    }

    pub fn next_lexicographical(&self) -> Option<Self> {
        self.0.next_lexicographical().map(Self)
    }

    pub fn into_inner(self) -> [u8; 32] {
        self.0.into_inner()
    }
}

impl fmt::Debug for TransactionEventsDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TransactionEventsDigest").field(&self.0).finish()
    }
}

impl AsRef<[u8]> for TransactionEventsDigest {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

// impl AsRef<[u8; 32]> for TransactionEventsDigest {
//     fn as_ref(&self) -> &[u8; 32] {
//         self.0.as_ref()
//     }
// }

impl std::str::FromStr for TransactionEventsDigest {
    type Err = RebasedError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [0; 32];
        let buffer = Base58::decode(s).map_err(|e| RebasedError::DigestsError(format!("{:?}", e)))?;
        if buffer.len() != 32 {
            return Err(RebasedError::DigestsError(format!(
                "Invalid digest length. Expected 32 bytes"
            )));
        }
        result.copy_from_slice(&buffer);
        Ok(Self::new(result))
    }
}
