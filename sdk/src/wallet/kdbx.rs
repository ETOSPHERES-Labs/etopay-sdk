use etopay_wallet::bip39::Mnemonic;
use kdbx_rs::database::Entry;
use kdbx_rs::errors::FailedUnlock;
use kdbx_rs::{CompositeKey, Database, Kdbx};
use log::info;
use secrecy::{ExposeSecret, SecretString};

/// load mnemonic from kdbx file
pub fn load_mnemonic(backup: &[u8], password: &SecretString) -> Result<Mnemonic, KdbxStorageError> {
    info!("Loading kdbx file from bytes");
    let kdbx = kdbx_rs::from_reader(backup)?;
    let key = CompositeKey::from_password(password.expose_secret());
    let unlocked = kdbx.unlock(&key)?;

    let Some(entry) = unlocked.find_entry(|entry| entry.title() == Some("mnemonic")) else {
        return Err(KdbxStorageError::NotFound("Entry not found".to_string()));
    };

    let Some(mnemonic) = entry.password() else {
        return Err(KdbxStorageError::NotFound("Mnemonic not found".to_string()));
    };

    let mnemonic = Mnemonic::from_phrase(mnemonic, etopay_wallet::bip39::Language::English)?;

    Ok(mnemonic)
}

/// store mnemonic in kdbx file
pub fn store_mnemonic(mnemonic: &Mnemonic, password: &SecretString) -> Result<Vec<u8>, KdbxStorageError> {
    info!("Creating kdbx file from mnemonic");

    let mut database = Database::default();
    database.set_name("etopay");

    let mut entry = Entry::default();
    entry.set_title("mnemonic");
    entry.set_password(mnemonic.phrase());
    database.add_entry(entry);

    let mut kdbx = Kdbx::from_database(database);
    kdbx.set_key(CompositeKey::from_password(password.expose_secret()))?;

    let mut buffer = Vec::new();

    kdbx.write(&mut buffer)?;

    Ok(buffer)
}

/// Wrapper for kdbx storage errors
#[derive(Debug, thiserror::Error)]
pub enum KdbxStorageError {
    /// Kdbx storage errors
    #[error("KdbxError: {0}")]
    KdbxError(#[from] kdbx_rs::Error),
    /// Kdbx open errors
    #[error("OpenError: {0}")]
    OpenError(#[from] kdbx_rs::errors::OpenError),
    /// Kdbx write errors
    #[error("WriteError: {0}")]
    WriteError(#[from] kdbx_rs::errors::WriteError),
    /// Kdbx unlock errors
    #[error("UnlockError: {0}")]
    UnlockError(#[from] kdbx_rs::errors::UnlockError),
    /// Kdbx key generation errors
    #[error("KeyGenerationError: {0}")]
    KeyGenerationError(#[from] kdbx_rs::errors::KeyGenerationError),
    /// Not found errors
    #[error("Not found: {0}")]
    NotFound(String),

    /// Error occurred while handling bip39 compliant mnemonics
    #[error("Bip39 error: {0:?}")]
    Bip39(#[from] etopay_wallet::bip39::ErrorKind),
}

impl From<FailedUnlock> for KdbxStorageError {
    fn from(funlock: FailedUnlock) -> KdbxStorageError {
        KdbxStorageError::UnlockError(funlock.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use secrecy::SecretString;

    #[test]
    fn test_store_and_load_mnemonic() {
        // Arrange
        let mnemonic = Mnemonic::new(
            etopay_wallet::bip39::MnemonicType::Words24,
            etopay_wallet::bip39::Language::English,
        );

        let password = SecretString::new("password".into());

        // Act
        let kdbx = store_mnemonic(&mnemonic, &password).unwrap();
        let mnemonic_recovered = load_mnemonic(&kdbx, &password).unwrap();

        // Assert
        assert_eq!(mnemonic_recovered.phrase(), mnemonic.phrase());
    }
}
