// Copyright (c) 2022, Mysten Labs, Inc.
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

//! Inspired by https://github.com/MystenLabs/fastcrypto/blob/main/fastcrypto/src/ed25519.rs

use std::{cell::OnceCell, fmt};

// it seems ed25519_consensus still uses an old version of rand, so we takeit from aes_gcm crate :D
use aes_gcm::aead::rand_core::{CryptoRng, RngCore};
use fastcrypto::encoding::{Base64, Encoding};

use super::super::RebasedError;

use super::super::traits::ToFromBytes;

/// Trait impl'd by a key/keypair that can create signatures.
///
pub trait Signer<Sig> {
    /// Create a new signature over a message.
    fn sign(&self, msg: &[u8]) -> Sig;
}

/// The length of a private key in bytes.
pub const ED25519_PRIVATE_KEY_LENGTH: usize = 32;

/// The length of a public key in bytes.
pub const ED25519_PUBLIC_KEY_LENGTH: usize = 32;

/// The length of a signature in bytes.
pub const ED25519_SIGNATURE_LENGTH: usize = 64;

/// The key pair bytes length is the same as the private key length. This enforces deserialization to always derive the public key from the private key.
pub const ED25519_KEYPAIR_LENGTH: usize = ED25519_PRIVATE_KEY_LENGTH;

/// Ed25519 public key.
#[derive(Clone, PartialEq, Eq)]
pub struct Ed25519PublicKey(pub ed25519_consensus::VerificationKey);

/// Ed25519 private key.
#[derive(zeroize::ZeroizeOnDrop)]
pub struct Ed25519PrivateKey(pub ed25519_consensus::SigningKey);

/// Ed25519 key pair.
#[derive(Debug, PartialEq, Eq)]
pub struct Ed25519KeyPair {
    public: Ed25519PublicKey,
    private: Ed25519PrivateKey,
}

/// Ed25519 signature.
#[derive(Debug, Clone)]
pub struct Ed25519Signature {
    pub sig: ed25519_consensus::Signature,
    // Helps implementing AsRef<[u8]>.
    pub bytes: OnceCell<[u8; ED25519_SIGNATURE_LENGTH]>,
}

impl Ed25519Signature {
    pub const LENGTH: usize = ED25519_SIGNATURE_LENGTH;
}

impl AsRef<[u8]> for Ed25519Signature {
    fn as_ref(&self) -> &[u8] {
        self.bytes.get_or_init::<_>(|| self.sig.to_bytes())
    }
}

//
// Implementation of [Ed25519PrivateKey].
//

impl AsRef<[u8]> for Ed25519PrivateKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl PartialEq for Ed25519PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl Eq for Ed25519PrivateKey {}

impl fmt::Display for Ed25519PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519PrivateKey[]")
    }
}
impl fmt::Debug for Ed25519PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ed25519PrivateKey[]")
    }
}

// impl SigningKey for Ed25519PrivateKey {
//     type PubKey = Ed25519PublicKey;
//     type Sig = Ed25519Signature;
//     const LENGTH: usize = ED25519_PRIVATE_KEY_LENGTH;
// }
//
impl ToFromBytes for Ed25519PrivateKey {
    fn from_bytes(bytes: &[u8]) -> Result<Self, RebasedError> {
        ed25519_consensus::SigningKey::try_from(bytes)
            .map(Ed25519PrivateKey)
            .map_err(|_| RebasedError::InvalidCryptoInput)
    }
}
//

//
// Implementation of [Ed25519PublicKey].
//

impl Ed25519PublicKey {
    pub const LENGTH: usize = ED25519_PUBLIC_KEY_LENGTH;
}

impl fmt::Debug for Ed25519PublicKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Base64::encode(self.as_ref()))
    }
}

impl AsRef<[u8]> for Ed25519PublicKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl<'a> From<&'a Ed25519PrivateKey> for Ed25519PublicKey {
    fn from(private: &'a Ed25519PrivateKey) -> Self {
        Ed25519PublicKey(private.0.verification_key())
    }
}

//
// Implementation of [Ed25519KeyPair].
//

impl Ed25519KeyPair {
    pub fn public(&'_ self) -> &'_ Ed25519PublicKey {
        &self.public
    }

    pub fn generate<R: CryptoRng + RngCore>(rng: &mut R) -> Self {
        let kp = ed25519_consensus::SigningKey::new(rng);
        Ed25519KeyPair {
            public: Ed25519PublicKey(kp.verification_key()),
            private: Ed25519PrivateKey(kp),
        }
    }
}

impl From<Ed25519PrivateKey> for Ed25519KeyPair {
    fn from(private: Ed25519PrivateKey) -> Self {
        let public = Ed25519PublicKey::from(&private);
        Ed25519KeyPair { public, private }
    }
}

impl From<ed25519_consensus::SigningKey> for Ed25519KeyPair {
    fn from(kp: ed25519_consensus::SigningKey) -> Self {
        Ed25519KeyPair {
            public: Ed25519PublicKey(kp.verification_key()),
            private: Ed25519PrivateKey(kp),
        }
    }
}

impl Signer<Ed25519Signature> for Ed25519KeyPair {
    fn sign(&self, msg: &[u8]) -> Ed25519Signature {
        Ed25519Signature {
            sig: self.private.0.sign(msg),
            bytes: OnceCell::new(),
        }
    }
}
