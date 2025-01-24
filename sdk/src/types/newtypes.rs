//! This module contains newtype wrappers for sensitive values. There are several reasons for this:
//! - We can make sure the information (such as passwords and pins) is not logged or sent to the
//!   backend by mistake.
//! - We control the creation and deletion of the objects such that we can ensure passwords and
//!   pins are non-empty, and also clear / [`zeroize`] the values once they are Dropped.
//! - We have control over the encrypt - decrypt lifetime of the password, making sure each value
//!   can only be used where it is explicitly intended to be used, and the encryption is only
//!   implemented and tested in one place.
//!
//! In general the implementation follows this pattern:
//! - A `struct` that (privately) wraps the underlying value is defined.
//! - We derive [`zeroize::Zeroize`] and [`zeroize::ZeroizeOnDrop`] to make sure the value is
//!   overwritten with their zero value once `Drop`ed.
//! - We implement [`core::fmt::Debug`] that does not print the value itself but rather a
//! placeholder value. This ensures the value cannot be printed or logged.
//! - We define (fallible) constructor functions that make sure the inner value is valid or return
//!   an Error. Additionally `unsafe` constructors can be used for circumventing the validations
//!   (eg. for use in tests).
//! - We define functions to get the inner value in safe ways (eg. as a [`secrecy::Secret`], or functions
//!   to convert (eg. for the encrypt / decrypt methods) to other newtypes.

use super::error::{Result, TypeError};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use iota_sdk::crypto::hashes::blake2b::Blake2b256;
use iota_sdk::crypto::hashes::Digest;
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};

macro_rules! impl_redacted_debug {
    ($type:ty) => {
        impl core::fmt::Debug for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}(<REDACTED>)", stringify!($type))
            }
        }
    };
}

/// A password that is not encrypted and stored as plain text.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop, Clone)]
pub struct PlainPassword(String);
impl_redacted_debug!(PlainPassword);

impl PlainPassword {
    /// Try to construct a new [`PlainPassword`] from a [`String`]-like value.
    pub fn try_from_string(password: impl Into<String>) -> Result<Self> {
        let password: String = password.into();
        if password.is_empty() {
            return Err(TypeError::EmptyPassword);
        }

        Ok(Self(password))
    }

    /// Encrypt this password with the provided pin and salt.
    pub fn encrypt(&self, pin: &EncryptionPin, salt: &EncryptionSalt) -> Result<EncryptedPassword> {
        let key = Blake2b256::new()
            .chain_update(pin.0.as_ref())
            .chain_update(salt.0.as_ref())
            .finalize();

        let Ok(cipher) = Aes256Gcm::new_from_slice(&key) else {
            return Err(TypeError::PasswordEncryption);
        };

        let nonce = Nonce::from_slice(salt.0.as_ref()); // 96-bits; unique per message
        let Ok(cipher) = cipher.encrypt(nonce, self.0.as_ref()) else {
            return Err(TypeError::PasswordEncryption);
        };

        Ok(EncryptedPassword(cipher.into()))
    }

    /// Helper function to convert into [`secrecy::Secret`] using cloning.
    pub fn into_secret(&self) -> secrecy::SecretBox<[u8]> {
        secrecy::SecretBox::new(self.0.as_bytes().into())
    }

    /// Helper function to convert into [`secrecy::Secret`] using cloning.
    pub fn into_secret_string(&self) -> secrecy::SecretString {
        self.0.clone().into()
    }

    /// Helper function to get the underlying string, use with caution!
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl TryFrom<String> for PlainPassword {
    type Error = TypeError;
    fn try_from(value: String) -> Result<Self> {
        Self::try_from_string(value)
    }
}
impl TryFrom<&str> for PlainPassword {
    type Error = TypeError;
    fn try_from(value: &str) -> Result<Self> {
        Self::try_from_string(value)
    }
}

/// An encrypted password.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop, Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EncryptedPassword(Box<[u8]>);
impl_redacted_debug!(EncryptedPassword);

impl EncryptedPassword {
    /// Decrypt this password with the provided pin and salt.
    /// Returns an error if the pin or salt is incorrect.
    pub fn decrypt(&self, pin: &EncryptionPin, salt: &EncryptionSalt) -> Result<PlainPassword> {
        let key = Blake2b256::new()
            .chain_update(pin.0.as_ref())
            .chain_update(salt.0.as_ref())
            .finalize();

        let Ok(cipher) = Aes256Gcm::new_from_slice(&key) else {
            return Err(TypeError::PasswordEncryption);
        };

        let nonce = Nonce::from_slice(salt.0.as_ref()); // 96-bits; unique per message
        let Ok(plaintext) = cipher.decrypt(nonce, self.0.as_ref()) else {
            return Err(TypeError::InvalidPinOrPassword);
        };

        Ok(PlainPassword(String::from_utf8_lossy(&plaintext).to_string()))
    }

    /// Create a new `EncryptedPassword` from raw bytes.
    ///
    /// # Safety
    ///
    /// This is `unsafe` since the bytes might not have come from an encryption step at all.
    ///
    pub unsafe fn new_unchecked(bytes: impl Into<Vec<u8>>) -> Self {
        Self(bytes.into().into())
    }
}

/// A non-empty pin used to encrypt the password.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
pub struct EncryptionPin(Box<[u8]>);
impl_redacted_debug!(EncryptionPin);

impl EncryptionPin {
    /// Try to construct a new [`EncryptionPin`] from a [`String`]-like value.
    pub fn try_from_string(pin: impl Into<String>) -> Result<Self> {
        let pin: String = pin.into();
        if pin.is_empty() {
            return Err(TypeError::EmptyPin);
        }
        Ok(Self(pin.as_bytes().into()))
    }
}
impl TryFrom<String> for EncryptionPin {
    type Error = TypeError;
    fn try_from(value: String) -> Result<Self> {
        Self::try_from_string(value)
    }
}
impl TryFrom<&str> for EncryptionPin {
    type Error = TypeError;
    fn try_from(value: &str) -> Result<Self> {
        Self::try_from_string(value)
    }
}

/// A salt used in the encryption process.
/// Should be unique for each encryption but is not a secret.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop, Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct EncryptionSalt(Box<[u8; 12]>);
impl_redacted_debug!(EncryptionSalt);

impl EncryptionSalt {
    /// Generate a new random [`EncryptionSalt`].
    pub fn generate() -> Self {
        let mut salt = [0u8; 12];
        OsRng.fill_bytes(&mut salt);
        Self(Box::new(salt))
    }
}

impl From<[u8; 12]> for EncryptionSalt {
    fn from(value: [u8; 12]) -> Self {
        Self(value.into())
    }
}

/// Simple wrapper around a non-empty access token that cannot be printed or logged, and is
/// automatically zeroized when dropped.
#[derive(zeroize::Zeroize, zeroize::ZeroizeOnDrop, Deserialize, Serialize, Clone)]
pub struct AccessToken(String);
impl_redacted_debug!(AccessToken);

impl AccessToken {
    /// Construct a new [`AccessToken`] from a [`String`], returns an error if the string is empty.
    pub fn try_from_string(token: impl Into<String>) -> Result<Self> {
        let token: String = token.into();
        if token.is_empty() {
            return Err(TypeError::EmptyAccessToken);
        }

        Ok(Self(token))
    }

    /// Helper function to get access to the inner String. Use with caution!
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for AccessToken {
    type Error = TypeError;
    fn try_from(value: String) -> Result<Self> {
        Self::try_from_string(value)
    }
}
impl TryFrom<&str> for AccessToken {
    type Error = TypeError;
    fn try_from(value: &str) -> Result<Self> {
        Self::try_from_string(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::newtypes::PlainPassword;

    #[test]
    fn test_debug_is_redacted() {
        let debug = format!("{:?}", PlainPassword::try_from_string("hello").unwrap());
        assert!(!debug.contains("hello"));
    }

    #[test]
    fn test_encrypt_password_success() {
        let password = PlainPassword::try_from_string("strong_password").unwrap();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let salt = EncryptionSalt::generate();

        let encrypted_password = password.encrypt(&pin, &salt).unwrap();
        assert!(!encrypted_password.0.is_empty());
    }

    #[test]
    fn test_decrypt_password_success() {
        let password = PlainPassword::try_from_string("strong_password").unwrap();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let salt = EncryptionSalt::generate();

        let encrypted_password = password.encrypt(&pin, &salt).unwrap();
        let decrypted_password = encrypted_password.decrypt(&pin, &salt).unwrap();

        assert_eq!(decrypted_password.0, password.0);
    }

    #[test]
    fn test_encrypt_password_with_special_characters() {
        let password = PlainPassword::try_from_string("strong_password!@#$%^&*()").unwrap();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let salt = EncryptionSalt::generate();

        let encrypted_password = password.encrypt(&pin, &salt).unwrap();
        let decrypted_password = encrypted_password.decrypt(&pin, &salt).unwrap();

        assert_eq!(decrypted_password.0, password.0);
    }

    #[test]
    fn test_decrypt_password_failure_wrong_pin() {
        let password = PlainPassword::try_from_string("strong_password").unwrap();
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let wrong_pin = EncryptionPin::try_from_string("54321").unwrap();
        let salt = EncryptionSalt::generate();

        let encrypted_password = password.encrypt(&pin, &salt).unwrap();
        let decrypted_password = encrypted_password.decrypt(&wrong_pin, &salt);

        decrypted_password.unwrap_err();
    }

    #[test]
    fn test_decrypt_password_failure_invalid_data() {
        let pin = EncryptionPin::try_from_string("12345").unwrap();
        let salt = EncryptionSalt::generate();

        // SAFETY: this is only for testing purposes to make sure an invalid encrypted password gives an error
        let invalid_encrypted_password = unsafe { EncryptedPassword::new_unchecked(b"invalid_encrypted_password") };

        let decrypted_password = invalid_encrypted_password.decrypt(&pin, &salt);

        decrypted_password.unwrap_err();
    }

    #[test]
    fn test_generate_salt() {
        let salt = EncryptionSalt::generate();
        assert_eq!(salt.0.len(), 12);
    }
}
