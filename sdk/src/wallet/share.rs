use base64::{Engine as _, engine::general_purpose::STANDARD};
use iota_sdk::crypto::{
    hashes::{Digest, blake2b::Blake2b256},
    keys::bip39::Mnemonic,
};
use secrecy::{ExposeSecret, SecretBox, SecretSlice, SecretString};
use std::str::FromStr;

/// A share that can be used with other [`Share`] to construct the secret.
#[derive(Debug, Clone)] // for testing purposes we also derive PartialEq
#[cfg_attr(test, derive(PartialEq))]
pub struct Share {
    /// The type of the secret payload stored by the shares.
    payload_type: PayloadType,

    /// The encoding used for the share parts.
    encoding: Encoding,

    /// The encryption method used to encrypt the data field.
    encryption: Encryption,

    /// The actual share data bytes, representing the `payload_type` content split into shares using
    /// `encoding` and encrypted using `encryption`.
    data: ShareData,
}

/// A type that has the immutable data from the share, and will zeroize it on Drop
#[derive(zeroize::ZeroizeOnDrop, Clone)]
#[cfg_attr(test, derive(PartialEq))] // for testing purposes we also derive PartialEq
struct ShareData(Box<[u8]>);

/// Debug implementation that does not print the content
impl std::fmt::Debug for ShareData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED ")?;
        f.write_str(core::any::type_name::<Self>())?;
        f.write_str("]")
    }
}

impl FromStr for Share {
    type Err = ShareError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut parts = value.split('-');
        let version: PayloadType = parts.next().ok_or(ShareError::NotEnoughParts)?.parse()?;
        let encoding: Encoding = parts.next().ok_or(ShareError::NotEnoughParts)?.parse()?;
        let encryption: Encryption = parts.next().ok_or(ShareError::NotEnoughParts)?.parse()?;
        let data = parts.next().ok_or(ShareError::NotEnoughParts)?;
        let data = ShareData(STANDARD.decode(data)?.into());

        Ok(Share {
            payload_type: version,
            encoding,
            encryption,
            data,
        })
    }
}

impl Share {
    /// Format this [`Share`] to a string value, returned as a [`Secret`].
    pub fn to_string(&self) -> SecretString {
        let base64_data = STANDARD.encode(&self.data.0);
        format!(
            "{}-{}-{}-{}",
            self.payload_type, self.encoding, self.encryption, base64_data
        )
        .into()
    }

    /// Checks if the share is encrypted.
    pub fn is_encrypted(&self) -> bool {
        self.encryption != Encryption::None
    }

    #[cfg(test)]
    pub(crate) fn mock_share() -> Self {
        Share {
            payload_type: PayloadType::MnemonicEntropy,
            encoding: Encoding::RustySecrets,
            encryption: Encryption::None,
            data: ShareData("test".to_string().into_bytes().into()),
        }
    }
}

/// Version of the Share, used for allowing different formats in the future
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum PayloadType {
    /// Payload contains the raw entropy bytes stored in the mnemonic.
    MnemonicEntropy,
}

impl std::fmt::Display for PayloadType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MnemonicEntropy => write!(f, "ME"),
        }
    }
}

impl FromStr for PayloadType {
    type Err = ShareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ME" => Ok(Self::MnemonicEntropy),
            other => Err(ShareError::InvalidShareFormat(format!(
                "Unrecognized Payload type: `{}`",
                other
            ))),
        }
    }
}

/// The SSS scheme used to encode the shares
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Encoding {
    RustySecrets,
}

impl std::fmt::Display for Encoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RustySecrets => write!(f, "RS"),
        }
    }
}

impl FromStr for Encoding {
    type Err = ShareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RS" => Ok(Self::RustySecrets),
            other => Err(ShareError::InvalidShareFormat(format!(
                "Unrecognized Encoding: `{}`",
                other
            ))),
        }
    }
}

/// The encryption used or none of the share payload
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Encryption {
    None,
    AesGcm,
}

impl std::fmt::Display for Encryption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "N"),
            Self::AesGcm => write!(f, "AesGcm"),
        }
    }
}

impl FromStr for Encryption {
    type Err = ShareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::None),
            "AesGcm" => Ok(Self::AesGcm),
            other => Err(ShareError::InvalidShareFormat(format!(
                "Unrecognized Encryption: `{}`",
                other
            ))),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[allow(missing_docs)]
/// Error produced when working with [`Share`] objects.
pub enum ShareError {
    #[error("InvalidShareFormat: {0}")]
    InvalidShareFormat(String),

    #[error("Not enough parts are available to parse share")]
    NotEnoughParts,

    #[error("No password was provided but is needed")]
    PasswordNotProvided,

    #[error("Provided {provided} shares but at least {required} are required")]
    NotEnoughShares { provided: usize, required: usize },

    #[error("Provided shares are incompatible: {0}")]
    IncompatibleShares(String),

    #[error("Error while decrypting / encrypting: {0}")]
    EncryptionError(&'static str),

    #[error("Unable to parse: {0:#?}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("Base64 decoding failed: {0:#?}")]
    Base64Decode(#[from] base64::DecodeError),

    #[error("Error in RustySecrets: {0:?}")]
    RustySecretsError(#[from] rusty_secrets::errors::Error),
}

#[derive(Debug)]
/// Contains all the shares generated by splitting a secret
pub struct GeneratedShares {
    /// recovery share that the user should download and store safely
    pub recovery: Share,
    /// share to store locally
    pub local: Share,
    /// backup share that is shared with etopay backend, encrypted
    pub backup: Share,
}

/// Creates shares from a [`Mnemonic`] that can be resolved into a [`Mnemonic`] again when reconstructed.
pub fn create_shares_from_mnemonic(
    mnemonic: impl Into<Mnemonic>,
    password: &SecretSlice<u8>,
) -> super::error::Result<GeneratedShares> {
    let mnemonic: Mnemonic = mnemonic.into();

    // convert the mnemonic string into the raw entropy it encodes
    let entropy =
        iota_sdk::crypto::keys::bip39::wordlist::decode(&mnemonic, &iota_sdk::crypto::keys::bip39::wordlist::ENGLISH)?;

    let entropy_bytes: &[u8] = entropy.as_ref();
    create_shares_from_secret(PayloadType::MnemonicEntropy, &entropy_bytes.to_vec().into(), password)
        .map_err(Into::into)
}

/// Reconstruct a [`Mnemonic`] from the shares. Can be used to initialize a wallet using the
/// [`iota_sdk::client::secret::mnemonic::MnemonicSecretManager::try_from_mnemonic`] function.
pub fn reconstruct_mnemonic(
    shares: &[&Share],
    password: Option<&SecretSlice<u8>>,
) -> super::error::Result<SecretBox<Mnemonic>> {
    let (payload_type, secret) = reconstruct_secret(shares, password)?;
    match payload_type {
        PayloadType::MnemonicEntropy => Ok(SecretBox::new(Box::new(
            iota_sdk::crypto::keys::bip39::wordlist::encode(
                secret.expose_secret(),
                &iota_sdk::crypto::keys::bip39::wordlist::ENGLISH,
            )?,
        ))),
    }
}

/// Creates shares from any secret represented as a vector of bytes.
fn create_shares_from_secret(
    payload_type: PayloadType,
    secret: &SecretSlice<u8>,
    password: &SecretSlice<u8>,
) -> Result<GeneratedShares, ShareError> {
    let out = rusty_secrets::dss::ss1::split_secret(
        2,
        3,
        secret.expose_secret(),
        // we specify reproducibility since we want to be able to regenerate the local share from
        // the others, and we need the signatures to match
        rusty_secrets::dss::ss1::Reproducibility::seeded("etopay".to_owned().into_bytes()),
        &None,
    )?;

    let mut share_data_iter = out.into_iter().map(|s| Share {
        payload_type,
        encoding: Encoding::RustySecrets,
        encryption: Encryption::None,
        data: ShareData(s.into_string().into_bytes().into()),
    });

    let recovery = share_data_iter.next().ok_or(ShareError::NotEnoughParts)?;
    let local = share_data_iter.next().ok_or(ShareError::NotEnoughParts)?;
    let mut backup = share_data_iter.next().ok_or(ShareError::NotEnoughParts)?;

    // encrypt the backup / recovery share(s) with the password
    backup.encryption = Encryption::AesGcm;
    backup.data = encrypt_with_password(&backup.data, password)?;

    Ok(GeneratedShares {
        recovery,
        local,
        backup,
    })
}

/// Reconstruct the secret from provided shares.
fn reconstruct_secret(
    shares: &[&Share],
    password: Option<&SecretSlice<u8>>,
) -> Result<(PayloadType, SecretSlice<u8>), ShareError> {
    let Some(share) = shares.first() else {
        return Err(ShareError::NotEnoughShares {
            provided: shares.len(),
            required: 2,
        });
    };

    // make sure all shares have the same payload type
    let payload_type = share.payload_type;
    if !shares.iter().all(|s| s.payload_type == payload_type) {
        return Err(ShareError::IncompatibleShares(format!(
            "All shares must have the same PayloadType, first share is `{payload_type}`"
        )));
    }

    // make sure all shares have the same encoding
    let encoding = share.encoding;
    if !shares.iter().all(|s| s.encoding == encoding) {
        return Err(ShareError::IncompatibleShares(format!(
            "All shares must use the same Encoding, first share uses `{encoding}`"
        )));
    }

    // decrypt any encrypted shares with the password
    let share_data = shares
        .iter()
        .map(|&s| match s.encryption {
            Encryption::None => Ok(s.data.clone()),
            Encryption::AesGcm => {
                let Some(password) = password else {
                    return Err(ShareError::PasswordNotProvided);
                };
                decrypt_with_password(&s.data, password)
            }
        })
        .collect::<Result<Vec<ShareData>, ShareError>>()?;

    match encoding {
        Encoding::RustySecrets => {
            // use the rusty_secrets to get the secret back
            let rusty_secrets_shares = share_data
                .iter()
                .map(|s| rusty_secrets::dss::ss1::Share::from_string(&String::from_utf8_lossy(&s.0)))
                .collect::<Result<Vec<rusty_secrets::dss::ss1::Share>, _>>()?;

            let (secret, _access_structure, _metadata) =
                rusty_secrets::dss::ss1::recover_secret(&rusty_secrets_shares)?;

            Ok((payload_type, SecretBox::new(secret.into())))
        }
    }
}

fn encrypt_with_password(data: &ShareData, key: &SecretSlice<u8>) -> Result<ShareData, ShareError> {
    use aes_gcm::{
        Aes256Gcm, Key,
        aead::{Aead, AeadCore, KeyInit, OsRng},
    };

    // create a random nonce value (96-bit for AesGcm256) since it needs to be unique for each
    // encryption with the same key
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    // hash the key string with the nonce to use as encryption key
    let key = Blake2b256::new()
        .chain_update(key.expose_secret())
        .chain_update(nonce)
        .finalize();
    // panics if length is invalid but since we have provided exactly 256 bits it is fine
    let key = Key::<aes_gcm::Aes256Gcm>::from_slice(key.as_slice());

    let cipher = Aes256Gcm::new(key);

    // encrypt the data and prepend the nonce value (to use while decrypting)
    let encrypted = cipher
        .encrypt(&nonce, data.0.as_ref())
        .map_err(|_| ShareError::EncryptionError("Error encrypting share using password"))?;

    let mut data = nonce.to_vec();
    data.extend(encrypted);
    Ok(ShareData(data.into()))
}

fn decrypt_with_password(data: &ShareData, key: &SecretSlice<u8>) -> Result<ShareData, ShareError> {
    use aes_gcm::{
        Aes256Gcm, Key, Nonce,
        aead::{Aead, KeyInit},
    };

    let data = &data.0;

    // split the nonce and the data
    let nonce_bytes = 96 / 8;
    if data.len() <= nonce_bytes {
        return Err(ShareError::InvalidShareFormat(format!(
            "not enough data for decryption, need at least {nonce_bytes} but {} bytes provided",
            data.len()
        )));
    }
    let (nonce, data) = data.split_at(nonce_bytes);

    let nonce = Nonce::from_slice(nonce);

    // hash the key string and nonce to use as encryption key
    let key = Blake2b256::new()
        .chain_update(key.expose_secret())
        .chain_update(nonce)
        .finalize();
    // panics if length is invalid but since we have provided exactly 256 bits it is fine
    let key = Key::<aes_gcm::Aes256Gcm>::from_slice(key.as_slice());

    let cipher = Aes256Gcm::new(key);

    Ok(ShareData(
        cipher
            .decrypt(nonce, data)
            .map_err(|_| ShareError::EncryptionError("Error decrypting share using password"))?
            .into(),
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use secrecy::SecretBox;

    #[test]
    fn test_string_serialization() {
        let s = Share {
            payload_type: super::PayloadType::MnemonicEntropy,
            encoding: Encoding::RustySecrets,
            encryption: super::Encryption::None,
            data: ShareData("data".to_string().into_bytes().into()),
        };
        println!("{:?}, {}, {:?}", s.to_string(), s.to_string().expose_secret(), s);

        let parsed = s.to_string().expose_secret().parse::<Share>().unwrap();
        assert_eq!(parsed, s);
    }

    #[test]
    fn test_split_recover_secret() {
        let secret = SecretBox::new("secret".to_string().into_bytes().into());
        let password = SecretBox::new("password".to_string().into_bytes().into());

        let shares = create_shares_from_secret(PayloadType::MnemonicEntropy, &secret, &password).unwrap();

        assert_eq!(
            reconstruct_secret(&[&shares.backup, &shares.local], Some(&password))
                .unwrap()
                .1
                .expose_secret(),
            secret.expose_secret()
        );
        assert_eq!(
            reconstruct_secret(&[&shares.backup, &shares.recovery], Some(&password))
                .unwrap()
                .1
                .expose_secret(),
            secret.expose_secret()
        );
        assert_eq!(
            reconstruct_secret(&[&shares.recovery, &shares.local], None)
                .unwrap()
                .1
                .expose_secret(),
            secret.expose_secret()
        );
        assert_eq!(
            reconstruct_secret(&[&shares.recovery, &shares.local, &shares.backup], Some(&password))
                .unwrap()
                .1
                .expose_secret(),
            secret.expose_secret()
        );

        assert!(reconstruct_secret(&[&shares.recovery], Some(&password)).is_err());
        assert!(reconstruct_secret(&[&shares.local], Some(&password)).is_err());
        assert!(reconstruct_secret(&[&shares.backup], Some(&password)).is_err());
    }

    #[test]
    fn test_split_recover_local() {
        // Arrange
        let secret = SecretBox::new("my hex string".to_string().into_bytes().into());
        let password = SecretBox::new("password".to_string().into_bytes().into());

        let shares = create_shares_from_secret(PayloadType::MnemonicEntropy, &secret, &password).unwrap();

        // reconstruct using backup and recovery
        let (_, reconstructed_secret) =
            reconstruct_secret(&[&shares.backup, &shares.recovery], Some(&password)).unwrap();

        // now create shares again and make sure we
        let new_shares =
            create_shares_from_secret(PayloadType::MnemonicEntropy, &reconstructed_secret, &password).unwrap();

        // reconstruct using a mix of old and "new" shares
        let (_, final_secret) = reconstruct_secret(&[&shares.backup, &new_shares.local], Some(&password)).unwrap();

        assert_eq!(final_secret.expose_secret(), secret.expose_secret());
    }

    #[test]
    fn test_split_recover_mnemonic() {
        // Arrange
        let password = SecretBox::new("password".to_string().into_bytes().into());

        let mnemonic = iota_sdk::client::Client::generate_mnemonic().unwrap();

        // Perform and check
        let shares = create_shares_from_mnemonic(mnemonic.clone(), &password).unwrap();

        assert_eq!(
            reconstruct_mnemonic(&[&shares.backup, &shares.local], Some(&password))
                .unwrap()
                .expose_secret()
                .as_bytes(),
            mnemonic.as_bytes()
        );
        assert_eq!(
            reconstruct_mnemonic(&[&shares.backup, &shares.recovery], Some(&password))
                .unwrap()
                .expose_secret()
                .as_bytes(),
            mnemonic.as_bytes()
        );
        assert_eq!(
            reconstruct_mnemonic(&[&shares.recovery, &shares.local], None)
                .unwrap()
                .expose_secret()
                .as_bytes(),
            mnemonic.as_bytes()
        );
        assert_eq!(
            reconstruct_mnemonic(&[&shares.recovery, &shares.local, &shares.backup], Some(&password))
                .unwrap()
                .expose_secret()
                .as_bytes(),
            mnemonic.as_bytes()
        );

        assert!(reconstruct_mnemonic(&[&shares.recovery], Some(&password)).is_err());
        assert!(reconstruct_mnemonic(&[&shares.local], Some(&password)).is_err());
        assert!(reconstruct_mnemonic(&[&shares.backup], Some(&password)).is_err());
    }

    #[test]
    fn test_split_recover_mnemonic_example() {
        let password: SecretSlice<u8> = "mnemonic share password".to_string().into_bytes().into();

        // the shares below have been generated with this code:
        // let mnemonic = iota_sdk::client::Client::generate_mnemonic().unwrap();
        // println!("{}", mnemonic.to_string());
        // let shares = creae_shares_from_mnemonic(mnemonic.clone(), &password).unwrap();
        // println!(
        //     "{}\n{}\n{}\n\n",
        //     shares.recovery.to_string().expose_secret(),
        //     shares.backup.to_string().expose_secret(),
        //     shares.local.to_string().expose_secret(),
        // );

        // Perform and check
        let mnemonic_str = "carpet liberty rent fox panic length romance slide item verb parade expose boss ladder reason vacuum fortune drip lizard dice main gate enrich aisle";

        let shares = [
            "ME-RS-N-Mi0xLUNBRVFBaGdESXFBRStPVUZYZTJnMTdLRFY1L2pWRllQTHdtZ0dCWExJbitjTERReFRyRHArWGNVMG5yY3UyVmFONFEvZkVoeXNadm5qNFhmRDVIZXZ3eHB2bENTYnZIZTFtOTlXdjJwby8zVWl0d2VhMnVWOTZaejB5WmhEdHlkRDFYcEg1R0RIYXFvZDBpTHdpcDZ3d1k5T0VWdEJhZmtkUVRGaTNNM3gvY2dsK0FDWVQ5WG50TlJycnRtWFRTUGZ4MG54R1lVc0NWUnNKY3h5Q0JxSHBlRGVRekpSTlFxVldMNGpJU3JCZkFRcEpYMnJoT1o4OXM1V3VLaW5PWFd0YUZncTRnd2t1VzR0ZkJJZzVUMjFlaXpGNEpWNzlMcXFXSDZoY3N0Z1huYzZYWTJvZjRvaytlYnJWOFBmR1lOU1NxRWQ4VFpqUzlBL0h0clJGNThEbUdaL2Z2Nmp5MjJjS01hUWllK1ZqdFZ4OUJyblJjWThYYTgxWmNTWlF4YlFLbFQ3MC9tRk5aQlN4ZXNLTWVTU24vV2hycEs0OU80ZW4zRkZJVTJqd2lLcGwybHpHMk0vdThJTzRZSlNCL1B6aVp4cGczcVk5Z25PRHNQR2lDZGNyejErcTVhYUdoMDdXUGlISFg5K1VpbVJjRThZS1BBNXUwNTBkQ2l2eVM2a2VhZkpFalQ0UXkxcElPaFRUd3ZrMWxrR0ZmeWp3bVBqL3JMRGY4YUc3ZXZlVWQveGwxbzlKMnh5ckhvQW9heTNVNVpHYjFCZGJ2OGFGNHJLb2wwTkorUlZBSTZJSHJCUnE4OGxJeGtzSlFxTm9GQ3o5b051N011OTVkMUJpZ3ErNjZiYzBuTWcyWXZYQXdaMkh3RjAzS0xRWEFWYjZVekZ1Lzc0MjYraElNUlR1M01mZDZoa01vMllMVzlxSS9odlBsaWg4RG5qaUFTUG9Fbkx2cVFidVpXaVBnQ3h2c1F4eXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==",
            "ME-RS-AesGcm-K3vx+e6IF6BOUJ2DemvsdflQq2CbolcFqdazfapauZTHdY/Hovh5zC8s5Qmfb2tRmRaluRX1gxMZfGDP52rakFnZpOzOCNGyHiI/dsiFDFbty0fEheEw+p1LrOI4zNwy7NE7ZsK0C756ggVfrhCin2Yw0KA6pALFqfWnQokx5Q43pUFd6ZGD8fwathC4NGx/hVTi9lxA2L6ScNQY9V3bEie40MKdpLQ6ELsPq+38UVJtqIgE0wJs8fDKSIGJVEPvP6wbVa+oPB/uFl5h56YeuYB2UGHdMJ54DCEoUBSd5QGoeKwjIylrZ+wXzchPXhtAfaCmqlf0fmKi9f5FQGrFwH9drf5HFE5Z/JWQC1FMKJTeBZ2CgcFvCtHuVm8VNnhhes1fUc7gL8VNOqE25LHFFQp3fBfeHXRkCmX+PAU+1N8KU6SFX0XqDr5anKAMH6thViBdno2m6K9tzqyucUnfgHYgp/cc+XXo9Ffw7v6lVTW3ls9diZwdwcs9JYqoKhWAs9dVGPz0017glpeAz01moJDPSMkhZwQh9GGWvhyeTWE9T28NS1G3cOBkW0GbgmIDjKeDDXAOjDyN7Db0FFL3TRAXthFtRXjJyZD1Xu2quYyjz1ZG70ILp0rDzzDaikUPUt1TCsAz+8NfLwHKz+H4oPUGprdUqgBVSGOySH+lKZaUbN17qIXjEKg58jh686s6i4GTD7Ndf6Xqsdc00PRDlm+jHwK7bNvkqkcChQHockIaIi4ETHCz/jqrca7uY8RIABv9Ni46+Ix1CrNY4qCUhep9oYZBGSLy2fQWWNk2nZgbrkipwUbgoV1IJV/kWCQ6ycjGG005kv3AFb6sZyrnFbvT7sa/JCKlo8gcVtzXlrJJqiO/7Qb1nTfj9dLd+/4ihpmwpFwPmKHi6zrZjJ8FbaDGkXSg+a82RQqz/AsH10hBd/tSZeZ5chdwgxTouoGix99HZipTKXLiAqW7Mo0N93+atNb9EWeHPBfsVbwJ2shBT5030QrY2qQfhTb4GUl52vPQBvpjxCzjPlvCWzFMlO8wrCP1sJm5egEb0F6Fpa9H3blBdMcb2NuKJ2VfSQzuJrbLzirnX3X0Pbk93S2dE5vs/2xsL6fqV18EPkVXO1mQtqsM8sMF8o6G/PLILN268Ga7CwcCL3qnoaCvahN3sHbciy38UH6s5hRTDvV75nWDj4oIaByrYx+JdgSZ4sucAn/bEQJCDSTVQ3sYQbEJGxc+xImNWudEoxdCmKYZPDFhUEIfO8pQRHTX8ZHZST+m97kJMuPvWg49UlrGu2YE6KbkNBEz7cSoWOuWpbrNjv1I8XKf8Sd82dvRWn3ZDc/4GXXE5oscG8UHTlz3XIpWNNrpE+wmn+AvmU0+n5r4Nv0LOFrlqH8Z2DcfjGqAVJkQMWFriruEcsPOvRgvGUeUtjulxEwcqX/UVmE5871rx0C2aJhazTnLkzt9TDFTaAf7J7zkIkhvKx8AU2A=",
        ];

        let shares: Vec<Share> = shares.iter().map(|&s| s.parse::<Share>().unwrap()).collect();
        let shares: Vec<&Share> = shares.iter().collect();

        assert_eq!(
            reconstruct_mnemonic(&shares, Some(&password))
                .unwrap()
                .expose_secret()
                .to_string(),
            mnemonic_str,
        );

        let shares = [
            "ME-RS-AesGcm-k0X9b0HVeq8HrVh8hAIs4LFD57alOXQ9BwXiAnHsOhmb6JmnD0w85Lchg65dedqbA2D+C7TFod2izbKcXw+k+rEEoPPFQDKDC/SjYORJnqNIOjAii8VNF714jAUqOMNXgLeXlLBWf1ExHxqLyzG81VGJMjcaNo1Z+sMnvsVCp+RbdZm4iOZweBqaftkX0xXTA7Nn5uxEHdblm6Z9KzvHnWYpx/uwX7XMk60mHoLQ6FoB+Jj8sq10Cy6eMolDly1MDD3+ynCt68Cswfr2iJGUOjF2Gebgdb5CkefbGX1mMLmDHC+coi6hUyj5+7WATNd+avGjTL37r+j4tX523H4QQBcvu459x/P5OhB5vP2qSO6oNe12ACv0n6j1qjU1qBLr/OUu870/uWpXFKi7cDKDokST9uAz+2t5N0kez1T3P3BsjLtJouWqsc64C0G+/qwX7UHaNe3cUiu9J6I8aLTtbDwK3PGRQdlcZ4Y0Gb2MaxwJsg85G0iHxzM1ODJqkRQK48Q5xPXfMqztjjxf355/T47yHanmfvq4p44osJSC8FvFB03BFzT7/WspUcTL3BKZNIAvNQf04T+NHdliM7x5kg4J0Ctwk8YB/h3HtmBqyTJym51WOsNvzHACWjRt64MjOYWy04sgjj4i3vrrkQTs93bUuH4bZ/1utFjqiHho2PaE9TOgyMP5y36LWhQRJEHoLTJXVTsJX5uRbqGoUX5OWWSEMuAxg8VrdLxdsQpikbjranf1ywIAA8vLK/HSNQJM/DbXUOh9W61yLJp35ONQEAmOC+l9HnYJrwvcQVCxMaL40D/JT7AkZmya9V2+SsO8wz+mjZT0pfEGmk7c/o8I/3/rYjJoISAwipz6hSol44IdPh57V3SbTxx0nykcVRAYP0/UIPWo4dui22LlnMUOwuH+WAJFmp+Dr/AK6PTxyaz0ILcq9w+7wTRx/Hi2AmFnPWiSLTxEPw8l3SS4CXnT6o7/vWk/YgN1pJWCsI4vhhB1Nui43+VI6e3lkvdaKiLtIXmbb+CRrS89R0cGX3C8XU6E9/ai6y+TaxRCQxH9qtyeILH/qBz9ZmOjpExHWtn09MAukWkbfpBWF3p3DltHnkzXInPDpVDBCb4/9gY857LTQgo0tmtdpT9/pmdUbIqUe/0KhnYnLVkRkK2wkjB7OqfM6+vJ++vrI+2Zq0b+jgnT78Bq5xd8lwnyxiqlEp4BUbdt24v/XgM/vhXbiyastFVPbyOQ/XGjuAarC5CgvjfO9NYostkzlm1yeylRD8igxJxFbCnLUR7ANXs0BpkEQokHZZx0BN6K2E2XIta2TRDGzm8EY1V4cZ/zMYv/mO4wz6Z7yDDwPv5I+/xkbTnuun+G0kABI+L7gkgzg8RqHnor7aAjQkEcC85sYL6lnZP60mQVUY6pjKwqvaAvgQCfd4Nc3yYC/jnOdZ6FqkAunmYcnV/XCQYtjiTT6ItN75YJjRVrdLCee4wQFFQ=",
            "ME-RS-N-Mi0yLUNBSVFBaGdESXFBRWk3b296TFVtbzNscG1jZEIxMXFESnVHbUpRUGYxUk9nOG5WQVNkR1NSTE5YQk41VytwV1dUcWVNSnVOakN3ZVd4eFk0Ylh6Y0NWTlhzYWFLNmM5UEUxWHVFT2lNVW9BeHdQNkRtM3BCT29EVklHWlFYbXZxQk9FOVYyU3FCZTlsblVRcUZBak9SQUllQjFVcWEvdlJMbXN3VWNqZlNJN0pVQWgvL3ZKZVBvbUxGUVFYcjZVSUE5dnpsaDgxVVNjaXZHUXlnOVQydWRNS202RTdveXQvcVpGam1DdUlYSFlkR1FCdkxrK01oaEErVmh2NlM2a0FkSU5veWRGUlZTdXpnU25zT25wcUJxb21oRWZaNkdmb1dsaHM5UUFadXRmeUgzdkxRT0hQeXc1TEZLbUE3dnpOTTJmMkc2dGZaZGR1Q2dnT2gydmZCUnh1ZmJSdStHN2VGSGtLdnVoOW16ekQ3YUF2Z3BRbldKakdrai9paGcyR1EyQVlBWWkrM200SXR5V3J3Q1ZRNDZlUjJCRGpmZllQK3BOTFNnL2xKNlNmbUswd204R2cvL2ZpbVBHODF4alcrckdIQkczUTV4U1JnWnlUcmt0TEFBWlY0VndJOENzdTlmSUNlT0tTYm9UVTVrOFJoWnZRS0pUNnhGS1d1K0l3OTVoWUlUZUdmVGxLa1NtdW93WmVXcFI1TjIyQUEyWENaVWVqVVBLdHlQWUYxOVNGTjRjUDlvTURuUng3bkxkY3B6cGE3QmJWQm1jRGNhTENZVW1PeXJKcDQwK1hoekJHVmlxVnBJTS9qNTJJQTg1TSt1TDVtM0xNUk12UFc4cEliNkpVYVlKV0FXSldWV3JKdlpzUGJrdmh3T0NlOXo5VWJUTXE1WUJzOC9OcFJnN0F4L2lmSTJ5ZHdxbDRacXd4N29MM0ZrK0daK1FDeHRyTWJSK2oxTzhROXFXOEJ5eEcveXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==",
        ];
        let shares: Vec<Share> = shares.iter().map(|&s| s.parse::<Share>().unwrap()).collect();
        let shares: Vec<&Share> = shares.iter().collect();
        assert_eq!(
            reconstruct_mnemonic(&shares, Some(&password))
                .unwrap()
                .expose_secret()
                .to_string(),
            mnemonic_str
        );

        let shares = [
            "ME-RS-N-Mi0xLUNBRVFBaGdESXFBRStPVUZYZTJnMTdLRFY1L2pWRllQTHdtZ0dCWExJbitjTERReFRyRHArWGNVMG5yY3UyVmFONFEvZkVoeXNadm5qNFhmRDVIZXZ3eHB2bENTYnZIZTFtOTlXdjJwby8zVWl0d2VhMnVWOTZaejB5WmhEdHlkRDFYcEg1R0RIYXFvZDBpTHdpcDZ3d1k5T0VWdEJhZmtkUVRGaTNNM3gvY2dsK0FDWVQ5WG50TlJycnRtWFRTUGZ4MG54R1lVc0NWUnNKY3h5Q0JxSHBlRGVRekpSTlFxVldMNGpJU3JCZkFRcEpYMnJoT1o4OXM1V3VLaW5PWFd0YUZncTRnd2t1VzR0ZkJJZzVUMjFlaXpGNEpWNzlMcXFXSDZoY3N0Z1huYzZYWTJvZjRvaytlYnJWOFBmR1lOU1NxRWQ4VFpqUzlBL0h0clJGNThEbUdaL2Z2Nmp5MjJjS01hUWllK1ZqdFZ4OUJyblJjWThYYTgxWmNTWlF4YlFLbFQ3MC9tRk5aQlN4ZXNLTWVTU24vV2hycEs0OU80ZW4zRkZJVTJqd2lLcGwybHpHMk0vdThJTzRZSlNCL1B6aVp4cGczcVk5Z25PRHNQR2lDZGNyejErcTVhYUdoMDdXUGlISFg5K1VpbVJjRThZS1BBNXUwNTBkQ2l2eVM2a2VhZkpFalQ0UXkxcElPaFRUd3ZrMWxrR0ZmeWp3bVBqL3JMRGY4YUc3ZXZlVWQveGwxbzlKMnh5ckhvQW9heTNVNVpHYjFCZGJ2OGFGNHJLb2wwTkorUlZBSTZJSHJCUnE4OGxJeGtzSlFxTm9GQ3o5b051N011OTVkMUJpZ3ErNjZiYzBuTWcyWXZYQXdaMkh3RjAzS0xRWEFWYjZVekZ1Lzc0MjYraElNUlR1M01mZDZoa01vMllMVzlxSS9odlBsaWg4RG5qaUFTUG9Fbkx2cVFidVpXaVBnQ3h2c1F4eXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==",
            "ME-RS-N-Mi0yLUNBSVFBaGdESXFBRWk3b296TFVtbzNscG1jZEIxMXFESnVHbUpRUGYxUk9nOG5WQVNkR1NSTE5YQk41VytwV1dUcWVNSnVOakN3ZVd4eFk0Ylh6Y0NWTlhzYWFLNmM5UEUxWHVFT2lNVW9BeHdQNkRtM3BCT29EVklHWlFYbXZxQk9FOVYyU3FCZTlsblVRcUZBak9SQUllQjFVcWEvdlJMbXN3VWNqZlNJN0pVQWgvL3ZKZVBvbUxGUVFYcjZVSUE5dnpsaDgxVVNjaXZHUXlnOVQydWRNS202RTdveXQvcVpGam1DdUlYSFlkR1FCdkxrK01oaEErVmh2NlM2a0FkSU5veWRGUlZTdXpnU25zT25wcUJxb21oRWZaNkdmb1dsaHM5UUFadXRmeUgzdkxRT0hQeXc1TEZLbUE3dnpOTTJmMkc2dGZaZGR1Q2dnT2gydmZCUnh1ZmJSdStHN2VGSGtLdnVoOW16ekQ3YUF2Z3BRbldKakdrai9paGcyR1EyQVlBWWkrM200SXR5V3J3Q1ZRNDZlUjJCRGpmZllQK3BOTFNnL2xKNlNmbUswd204R2cvL2ZpbVBHODF4alcrckdIQkczUTV4U1JnWnlUcmt0TEFBWlY0VndJOENzdTlmSUNlT0tTYm9UVTVrOFJoWnZRS0pUNnhGS1d1K0l3OTVoWUlUZUdmVGxLa1NtdW93WmVXcFI1TjIyQUEyWENaVWVqVVBLdHlQWUYxOVNGTjRjUDlvTURuUng3bkxkY3B6cGE3QmJWQm1jRGNhTENZVW1PeXJKcDQwK1hoekJHVmlxVnBJTS9qNTJJQTg1TSt1TDVtM0xNUk12UFc4cEliNkpVYVlKV0FXSldWV3JKdlpzUGJrdmh3T0NlOXo5VWJUTXE1WUJzOC9OcFJnN0F4L2lmSTJ5ZHdxbDRacXd4N29MM0ZrK0daK1FDeHRyTWJSK2oxTzhROXFXOEJ5eEcveXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==",
        ];
        let shares: Vec<Share> = shares.iter().map(|&s| s.parse::<Share>().unwrap()).collect();
        let shares: Vec<&Share> = shares.iter().collect();
        assert_eq!(
            reconstruct_mnemonic(&shares, None).unwrap().expose_secret().to_string(),
            mnemonic_str
        );
    }

    #[test]
    fn test_aes_gcm_encrypt_decrypt() {
        let key: SecretSlice<u8> = "key".to_string().into_bytes().into();
        let data = ShareData("my secret data".as_bytes().to_vec().into());

        assert_eq!(
            decrypt_with_password(&encrypt_with_password(&data, &key).unwrap(), &key).unwrap(),
            data,
        );
    }

    #[test]
    fn test_aes_gcm_encrypt_decrypt_wrong_key() {
        let key: SecretSlice<u8> = "key".to_string().into_bytes().into();
        let wrong_key: SecretSlice<u8> = "wrong key".to_string().into_bytes().into();

        let data = ShareData("my secret data".as_bytes().to_vec().into());

        assert!(decrypt_with_password(&encrypt_with_password(&data, &key).unwrap(), &wrong_key).is_err());
    }

    #[test]
    fn test_aes_gcm_decrypt_examples() {
        let key: SecretSlice<u8> = "key three".to_string().into_bytes().into();
        let data = ShareData("secret".as_bytes().to_vec().into());

        // Tests to make sure old generated encrypted data is still recoverable. The three examples
        // are generated with this code:
        // println!("{:?}", encrypt_with_password(data, &key).unwrap());

        assert_eq!(
            decrypt_with_password(
                &ShareData(Box::new([
                    32, 112, 222, 26, 190, 160, 235, 203, 235, 74, 13, 213, 181, 30, 151, 28, 60, 146, 145, 37, 128,
                    57, 80, 202, 77, 21, 179, 21, 100, 60, 85, 127, 68, 223,
                ])),
                &key
            )
            .unwrap(),
            data
        );

        assert_eq!(
            decrypt_with_password(
                &ShareData(Box::new([
                    76, 61, 16, 170, 160, 112, 228, 107, 253, 241, 246, 102, 145, 90, 79, 73, 157, 173, 81, 106, 1,
                    200, 23, 180, 127, 225, 147, 226, 233, 110, 94, 50, 150, 110
                ])),
                &key
            )
            .unwrap(),
            data
        );

        assert_eq!(
            decrypt_with_password(
                &ShareData(Box::new([
                    138, 139, 139, 160, 101, 108, 251, 7, 211, 55, 8, 160, 244, 248, 42, 23, 172, 229, 68, 143, 129,
                    245, 6, 117, 192, 226, 109, 184, 0, 84, 68, 165, 143, 201
                ])),
                &key
            )
            .unwrap(),
            data
        );
    }
}
