// Copyright (c) 2022, Mysten Labs, Inc.
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// From https://github.com/MystenLabs/fastcrypto/blob/main/fastcrypto/src/hash.rs

use std::fmt;

use digest::{generic_array::GenericArray, typenum};
use fastcrypto::encoding::{Base64, Encoding};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

/// Represents a digest of `DIGEST_LEN` bytes.
#[serde_as]
#[derive(Hash, PartialEq, Eq, Clone, Serialize, Deserialize, Ord, PartialOrd, Copy)]
pub struct Digest<const DIGEST_LEN: usize> {
    #[serde_as(as = "[_; DIGEST_LEN]")]
    pub digest: [u8; DIGEST_LEN],
}

impl<const DIGEST_LEN: usize> Digest<DIGEST_LEN> {
    /// Create a new digest containing the given bytes
    pub fn new(digest: [u8; DIGEST_LEN]) -> Self {
        Digest { digest }
    }

    /// The size of this digest in bytes.
    pub fn size(&self) -> usize {
        DIGEST_LEN
    }
}

impl<const DIGEST_LEN: usize> fmt::Debug for Digest<DIGEST_LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", Base64::encode(self.digest))
    }
}

impl<const DIGEST_LEN: usize> fmt::Display for Digest<DIGEST_LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", Base64::encode(self.digest))
    }
}

impl<const DIGEST_LEN: usize> AsRef<[u8]> for Digest<DIGEST_LEN> {
    fn as_ref(&self) -> &[u8] {
        self.digest.as_ref()
    }
}

impl<const DIGEST_LEN: usize> From<Digest<DIGEST_LEN>> for [u8; DIGEST_LEN] {
    fn from(digest: Digest<DIGEST_LEN>) -> Self {
        digest.digest
    }
}

/// Trait implemented by hash functions providing a output of fixed length
pub trait HashFunction<const DIGEST_LENGTH: usize>: Default {
    /// The length of this hash functions digests in bytes.
    const OUTPUT_SIZE: usize = DIGEST_LENGTH;

    /// Create a new hash function of the given type
    fn new() -> Self {
        Self::default()
    }

    /// Process the given data, and update the internal of the hash function.
    fn update<Data: AsRef<[u8]>>(&mut self, data: Data);

    /// Retrieve result and consume hash function.
    fn finalize(self) -> Digest<DIGEST_LENGTH>;

    /// Compute the digest of the given data and consume the hash function.
    fn digest<Data: AsRef<[u8]>>(data: Data) -> Digest<DIGEST_LENGTH> {
        let mut h = Self::default();
        h.update(data);
        h.finalize()
    }

    /// Compute a single digest from all slices in the iterator in order and consume the hash function.
    fn digest_iterator<K: AsRef<[u8]>, I: Iterator<Item = K>>(iter: I) -> Digest<DIGEST_LENGTH> {
        let mut h = Self::default();
        iter.for_each(|item| h.update(item));
        h.finalize()
    }
}

/// This trait is implemented by all messages that can be hashed.
pub trait Hash<const DIGEST_LEN: usize> {
    /// The type of the digest when this is hashed.
    type TypedDigest: Into<Digest<DIGEST_LEN>> + Eq + std::hash::Hash + Copy + fmt::Debug;

    fn digest(&self) -> Self::TypedDigest;
}

/// This wraps a [digest::Digest] as a [HashFunction].
#[derive(Default)]
pub struct HashFunctionWrapper<Variant, const DIGEST_LEN: usize>(Variant);

impl<Variant: digest::Digest + Default, const DIGEST_LEN: usize> HashFunction<DIGEST_LEN>
    for HashFunctionWrapper<Variant, DIGEST_LEN>
{
    fn update<Data: AsRef<[u8]>>(&mut self, data: Data) {
        self.0.update(data);
    }

    fn finalize(self) -> Digest<DIGEST_LEN> {
        let mut digest = [0u8; DIGEST_LEN];
        self.0.finalize_into(GenericArray::from_mut_slice(&mut digest));
        Digest { digest }
    }
}

// Impl std::io::Write for HashFunctionWrapper. Needed for compatibility in Sui.
impl<Variant: digest::Digest + Default, const DIGEST_LEN: usize> std::io::Write
    for HashFunctionWrapper<Variant, DIGEST_LEN>
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

/// The [BLAKE2-256](https://en.wikipedia.org/wiki/BLAKE_(hash_function)#BLAKE2) hash function with 256 bit digests.
pub type Blake2b256 = HashFunctionWrapper<blake2::Blake2b<typenum::U32>, 32>;
