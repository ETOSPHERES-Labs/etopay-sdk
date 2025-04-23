// https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/crypto.rs#L700

use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::hash::HashFunction;
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

use super::super::serde::Readable;
use super::IntentMessage;

/// Extracted from https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/crypto.rs#L1687
const ED25519_SIGNATURE_SCHEME_FLAG: u8 = 0x00;

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

    pub fn new_secure<T>(value: &IntentMessage<T>, secret: &dyn Signer<Signature>) -> Self
    where
        T: Serialize,
    {
        // Compute the BCS hash of the value in intent message. In the case of
        // transaction data, this is the BCS hash of `struct TransactionData`,
        // different from the transaction digest itself that computes the BCS
        // hash of the Rust type prefix and `struct TransactionData`.
        // (See `fn digest` in `impl Message for SenderSignedData`).
        let mut hasher = Blake2b256::default();
        hasher.update(bcs::to_bytes(&value).expect("Message serialization should not fail"));

        Signer::sign(secret, &hasher.finalize().digest)
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
                if x == &ED25519_SIGNATURE_SCHEME_FLAG {
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
        signature_bytes.extend_from_slice(&[ED25519_SIGNATURE_SCHEME_FLAG]);
        signature_bytes.extend_from_slice(sig.as_ref());
        signature_bytes.extend_from_slice(self.public().as_ref());
        let sign = Ed25519IotaSignature::from_bytes(&signature_bytes[..])
            .expect("Serialized signature did not have expected size");

        Signature::Ed25519IotaSignature(sign)
    }
}

// convenience from impl
impl From<Signature> for iota_sdk_rebased::types::crypto::Signature {
    fn from(value: Signature) -> Self {
        match value {
            Signature::Ed25519IotaSignature(ed25519_iota_signature) => Self::Ed25519IotaSignature(

                <iota_sdk_rebased::types::crypto::Ed25519IotaSignature as iota_sdk_rebased::types::crypto::ToFromBytes>::from_bytes(ed25519_iota_signature.as_bytes()).unwrap()


            )
        }
    }
}
