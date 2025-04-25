// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/crypto.rs#L700

use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::EncodeDecodeBase64;
use fastcrypto::traits::KeyPair;
use fastcrypto::{
    ed25519::{Ed25519PublicKey, Ed25519Signature},
    encoding::{Base64, Encoding},
    error::FastCryptoError,
    hash::Blake2b256,
    traits::{Authenticator, Signer, ToFromBytes, VerifyingKey},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{Bytes, serde_as};

use super::super::{RebasedError, serde::Readable};
use super::IntentMessage;

#[derive(Clone, Copy, Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum SignatureScheme {
    ED25519,
    // Secp256k1,
    // Secp256r1,
    // BLS12381, // This is currently not supported for user Iota Address.
    // MultiSig,
    // ZkLoginAuthenticator,
    // PasskeyAuthenticator,
}

impl SignatureScheme {
    pub fn flag(&self) -> u8 {
        match self {
            SignatureScheme::ED25519 => 0x00,
            // SignatureScheme::Secp256k1 => 0x01,
            // SignatureScheme::Secp256r1 => 0x02,
            // SignatureScheme::MultiSig => 0x03,
            // SignatureScheme::BLS12381 => 0x04, // This is currently not supported for user Iota
            // // Address.
            // SignatureScheme::ZkLoginAuthenticator => 0x05,
            // SignatureScheme::PasskeyAuthenticator => 0x06,
        }
    }

    /// Takes as input an hasher and updates it with a flag byte if the input
    /// scheme is not ED25519; it does nothing otherwise.
    pub fn update_hasher_with_flag(&self, hasher: &mut Blake2b256) {
        #[allow(unreachable_patterns)]
        match self {
            SignatureScheme::ED25519 => (),
            _ => hasher.update([self.flag()]),
        };
    }

    pub fn from_flag(flag: &str) -> Result<SignatureScheme, RebasedError> {
        let byte_int = flag
            .parse::<u8>()
            .map_err(|_| RebasedError::KeyConversion("Invalid key scheme".to_string()))?;
        Self::from_flag_byte(&byte_int)
    }

    pub fn from_flag_byte(byte_int: &u8) -> Result<SignatureScheme, RebasedError> {
        match byte_int {
            0x00 => Ok(SignatureScheme::ED25519),
            // 0x01 => Ok(SignatureScheme::Secp256k1),
            // 0x02 => Ok(SignatureScheme::Secp256r1),
            // 0x03 => Ok(SignatureScheme::MultiSig),
            // 0x04 => Ok(SignatureScheme::BLS12381),
            // 0x05 => Ok(SignatureScheme::ZkLoginAuthenticator),
            // 0x06 => Ok(SignatureScheme::PasskeyAuthenticator),
            _ => Err(RebasedError::KeyConversion("Invalid key scheme".to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Signature {
    Ed25519IotaSignature(Ed25519IotaSignature),
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bytes = self.as_ref();

        if serializer.is_human_readable() {
            let s = Base64::encode(bytes);
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_bytes(bytes)
        }
    }
}

impl<'de> Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;

        let bytes = if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;
            Base64::decode(&s).map_err(|e| Error::custom(e.to_string()))?
        } else {
            let data: Vec<u8> = Vec::deserialize(deserializer)?;
            data
        };

        Self::from_bytes(&bytes).map_err(|e| Error::custom(e.to_string()))
    }
}

impl Signature {
    /// The messaged passed in is already hashed form.
    pub fn new_hashed(hashed_msg: &[u8], secret: &dyn Signer<Signature>) -> Self {
        Signer::sign(secret, hashed_msg)
    }

    pub fn new_secure<T>(value: &IntentMessage<T>, secret: &dyn Signer<Signature>) -> Result<Self, RebasedError>
    where
        T: Serialize,
    {
        // Compute the BCS hash of the value in intent message. In the case of
        // transaction data, this is the BCS hash of `struct TransactionData`,
        // different from the transaction digest itself that computes the BCS
        // hash of the Rust type prefix and `struct TransactionData`.
        // (See `fn digest` in `impl Message for SenderSignedData`).
        let mut hasher = Blake2b256::default();
        hasher.update(bcs::to_bytes(&value)?);

        Ok(Signer::sign(secret, &hasher.finalize().digest))
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        match self {
            Signature::Ed25519IotaSignature(sig) => sig.as_ref(),
            // Signature::Secp256k1IotaSignature(sig) => sig.as_ref(),
            // Signature::Secp256r1IotaSignature(sig) => sig.as_ref(),
        }
    }
}
impl AsMut<[u8]> for Signature {
    fn as_mut(&mut self) -> &mut [u8] {
        match self {
            Signature::Ed25519IotaSignature(sig) => sig.as_mut(),
            // Signature::Secp256k1IotaSignature(sig) => sig.as_mut(),
            // Signature::Secp256r1IotaSignature(sig) => sig.as_mut(),
        }
    }
}

impl ToFromBytes for Signature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match bytes.first() {
            Some(x) => {
                if x == &Ed25519IotaSignature::SCHEME.flag() {
                    Ok(Signature::Ed25519IotaSignature(
                        <Ed25519IotaSignature as ToFromBytes>::from_bytes(bytes)?,
                    ))
                // } else if x == &Secp256k1IotaSignature::SCHEME.flag() {
                //     Ok(<Secp256k1IotaSignature as ToFromBytes>::from_bytes(bytes)?.into())
                // } else if x == &Secp256r1IotaSignature::SCHEME.flag() {
                //     Ok(<Secp256r1IotaSignature as ToFromBytes>::from_bytes(bytes)?.into())
                } else {
                    Err(FastCryptoError::InvalidInput)
                }
            }
            _ => Err(FastCryptoError::InvalidInput),
        }
    }
}

// Ed25519 Iota Signature port
//

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Ed25519IotaSignature(
    #[serde_as(as = "Readable<Base64, Bytes>")] [u8; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1],
);

impl Ed25519IotaSignature {
    const SCHEME: SignatureScheme = SignatureScheme::ED25519;
}

impl AsRef<[u8]> for Ed25519IotaSignature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
impl AsMut<[u8]> for Ed25519IotaSignature {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
// // Implementation useful for simplify testing when mock signature is needed
// impl Default for Ed25519IotaSignature {
//     fn default() -> Self {
//         Self([0; Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1])
//     }
// }
//
// impl IotaSignatureInner for Ed25519IotaSignature {
impl Ed25519IotaSignature {
    // type Sig = Ed25519Signature;
    // type PubKey = Ed25519PublicKey;
    // type KeyPair = Ed25519KeyPair;
    const LENGTH: usize = Ed25519PublicKey::LENGTH + Ed25519Signature::LENGTH + 1;
}

// impl IotaPublicKey for Ed25519PublicKey {
//     const SIGNATURE_SCHEME: SignatureScheme = SignatureScheme::ED25519;
// }

impl ToFromBytes for Ed25519IotaSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        if bytes.len() != Self::LENGTH {
            return Err(FastCryptoError::InputLengthWrong(Self::LENGTH));
        }
        let mut sig_bytes = [0; Self::LENGTH];
        sig_bytes.copy_from_slice(bytes);
        Ok(Self(sig_bytes))
    }
}

impl Signer<Signature> for Ed25519KeyPair {
    fn sign(&self, msg: &[u8]) -> Signature {
        let sig: Ed25519Signature = self.sign(msg);

        let mut signature_bytes: Vec<u8> = Vec::new();
        signature_bytes.extend_from_slice(&[Ed25519IotaSignature::SCHEME.flag()]);
        signature_bytes.extend_from_slice(sig.as_ref());
        signature_bytes.extend_from_slice(self.public().as_ref());
        #[allow(
            clippy::expect_used,
            reason = "the required length is constant, thus this is acceptable"
        )]
        let sign = Ed25519IotaSignature::from_bytes(&signature_bytes[..])
            .expect("Serialized signature did not have expected size");

        Signature::Ed25519IotaSignature(sign)
    }
}

/// Due to the incompatibility of [enum Signature] (which dispatches a trait
/// that assumes signature and pubkey bytes for verification), here we add a
/// wrapper enum where member can just implement a lightweight [trait
/// AuthenticatorTrait]. This way MultiSig (and future Authenticators) can
/// implement its own `verify`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericSignature {
    // MultiSig,
    Signature(Signature),
    // ZkLoginAuthenticator,
    // PasskeyAuthenticator,
}

impl From<Signature> for GenericSignature {
    fn from(value: Signature) -> Self {
        Self::Signature(value)
    }
}

/// GenericSignature encodes a single signature [enum Signature] as is `flag ||
/// signature || pubkey`. It encodes [struct MultiSigLegacy] as the MultiSig
/// flag (0x03) concat with the bcs serializedbytes of [struct MultiSigLegacy]
/// i.e. `flag || bcs_bytes(MultiSigLegacy)`. [struct Multisig] is encodede as
/// the MultiSig flag (0x03) concat with the bcs serializedbytes of [struct
/// Multisig] i.e. `flag || bcs_bytes(Multisig)`.
impl ToFromBytes for GenericSignature {
    fn from_bytes(bytes: &[u8]) -> Result<Self, FastCryptoError> {
        match SignatureScheme::from_flag_byte(bytes.first().ok_or(FastCryptoError::InputTooShort(0))?) {
            Ok(x) => match x {
                SignatureScheme::ED25519 => {
                    //| SignatureScheme::Secp256k1 | SignatureScheme::Secp256r1 => {
                    Ok(GenericSignature::Signature(
                        Signature::from_bytes(bytes).map_err(|_| FastCryptoError::InvalidSignature)?,
                    ))
                } // SignatureScheme::MultiSig => Ok(GenericSignature::MultiSig(MultiSig::from_bytes(bytes)?)),
                  // SignatureScheme::ZkLoginAuthenticator => {
                  //     let zk_login = ZkLoginAuthenticator::from_bytes(bytes)?;
                  //     Ok(GenericSignature::ZkLoginAuthenticator(zk_login))
                  // }
                  // SignatureScheme::PasskeyAuthenticator => {
                  //     let passkey = PasskeyAuthenticator::from_bytes(bytes)?;
                  //     Ok(GenericSignature::PasskeyAuthenticator(passkey))
                  // }
                  // _ => Err(FastCryptoError::InvalidInput),
            },
            Err(_) => Err(FastCryptoError::InvalidInput),
        }
    }
}

/// Trait useful to get the bytes reference for [enum GenericSignature].
impl AsRef<[u8]> for GenericSignature {
    fn as_ref(&self) -> &[u8] {
        match self {
            // GenericSignature::MultiSig(s) => s.as_ref(),
            GenericSignature::Signature(s) => s.as_ref(),
            // GenericSignature::ZkLoginAuthenticator(s) => s.as_ref(),
            // GenericSignature::PasskeyAuthenticator(s) => s.as_ref(),
        }
    }
}

impl ::serde::Serialize for GenericSignature {
    fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            #[derive(serde::Serialize)]
            struct GenericSignature(String);
            GenericSignature(self.encode_base64()).serialize(serializer)
        } else {
            #[derive(serde::Serialize)]
            struct GenericSignature<'a>(&'a [u8]);
            GenericSignature(self.as_ref()).serialize(serializer)
        }
    }
}

impl<'de> ::serde::Deserialize<'de> for GenericSignature {
    fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;

        if deserializer.is_human_readable() {
            #[derive(serde::Deserialize)]
            struct GenericSignature(String);
            let s = GenericSignature::deserialize(deserializer)?;
            Self::decode_base64(&s.0).map_err(::serde::de::Error::custom)
        } else {
            #[derive(serde::Deserialize)]
            struct GenericSignature(Vec<u8>);

            let data = GenericSignature::deserialize(deserializer)?;
            Self::from_bytes(&data.0).map_err(|e| Error::custom(e.to_string()))
        }
    }
}
