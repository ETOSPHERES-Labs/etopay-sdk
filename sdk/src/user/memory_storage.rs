//! This module provides the implementation of the `UserRepo` using a non-persistent in-memory
//! database. This implementation is mainly used on platforms that do not support file-access,
//! such as when compiling for the WASM target (see the WASM binding README for more information).

use super::error::{Result, UserKvStorageError};
use crate::types::users::UserEntity;
use std::collections::HashMap;

pub struct MemoryUserStorage {
    db: HashMap<String, UserEntity>,
}

/// Implementation of [`super::UserKvStorage`] using an in-memory database. Does not provide any type
/// of persistence, but does not require access to the file-system.
impl MemoryUserStorage {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

impl super::UserKvStorage for MemoryUserStorage {
    #[doc = " Get a value by key. Returns Error if key does not exist."]
    fn get(&self, username: &str) -> Result<UserEntity> {
        self.db
            .get(username)
            .ok_or(UserKvStorageError::UserNotFound {
                username: username.to_owned(),
            })
            .cloned()
    }

    #[doc = " Remove a key and the associated value. No error if the key does not exist."]
    fn delete(&mut self, username: &str) -> Result<()> {
        self.db.remove(username);
        Ok(())
    }

    #[doc = " Check if a key exists."]
    fn exists(&self, username: &str) -> Result<bool> {
        Ok(self.db.contains_key(username))
    }

    #[doc = " Associate a key with a value."]
    fn set(&mut self, username: &str, value: &UserEntity) -> Result<()> {
        self.db.insert(username.to_owned(), value.clone());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        types::{newtypes::EncryptionSalt, users::KycType},
        user::UserKvStorage,
    };

    fn create_user_entity() -> UserEntity {
        UserEntity {
            user_id: None,
            username: String::from("username"),
            encrypted_password: None,
            salt: EncryptionSalt::generate(),
            is_kyc_verified: false,
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            customer_details: None,

            local_share: None,
            wallet_transactions: Vec::new(),
        }
    }
    use testing::CleanUp;

    #[rstest_reuse::template]
    #[rstest::rstest]
    #[case::memory(Box::new(MemoryUserStorage::new()))]
    #[cfg_attr(
        feature = "jammdb_repo",
        case::jammdb(Box::new(crate::user::file_storage::FileUserStorage::new(&std::path::Path::new(&CleanUp::default().path_prefix)).unwrap()))
    )]
    fn all_backends(#[case] mut kv: Box<dyn UserKvStorage>) {}

    // store -> load
    #[rstest_reuse::apply(all_backends)]
    fn test_set_get(mut kv: Box<dyn UserKvStorage>) {
        // Arrange
        let user = create_user_entity();

        // Act
        kv.set(&user.username, &user).unwrap();
        let load = kv.get(&user.username).unwrap();

        // Assert
        assert_eq!(load, user);
    }

    // store overwrite
    #[rstest_reuse::apply(all_backends)]
    fn test_set_set_get(mut kv: Box<dyn UserKvStorage>) {
        // Arrange

        let mut user = create_user_entity();
        // Act
        kv.set(&user.username, &user).unwrap();

        user.is_kyc_verified = true;
        kv.set(&user.username, &user).unwrap();

        let load = kv.get(&user.username).unwrap();

        // Assert
        assert_eq!(load, user);
    }

    // load empty
    #[rstest_reuse::apply(all_backends)]
    fn test_get_nonexistant(kv: Box<dyn UserKvStorage>) {
        // Arrange

        // Act
        let load = kv.get("nonexistant_user");

        // Assert
        assert!(matches!(load.unwrap_err(), UserKvStorageError::UserNotFound { .. }));
    }

    // exists yes & no
    #[rstest_reuse::apply(all_backends)]
    fn test_exists_yes(mut kv: Box<dyn UserKvStorage>) {
        // Arrange
        let user = create_user_entity();
        kv.set(&user.username, &user).unwrap();

        // Act
        let load = kv.exists(&user.username).unwrap();

        // Assert
        assert!(load);
    }
    #[rstest_reuse::apply(all_backends)]
    fn test_exists_not(kv: Box<dyn UserKvStorage>) {
        // Arrange
        // Act
        let load = kv.exists("nonexistant_user").unwrap();

        // Assert
        assert!(!load);
    }

    // delete existant + get
    #[rstest_reuse::apply(all_backends)]
    fn test_delete_exists(mut kv: Box<dyn UserKvStorage>) {
        // Arrange
        let user = create_user_entity();
        kv.set(&user.username, &user).unwrap();

        // Act
        assert!(kv.exists(&user.username).unwrap());
        kv.delete(&user.username).unwrap();
        assert!(!kv.exists(&user.username).unwrap());
        assert!(kv.get(&user.username).is_err());
    }
    #[rstest_reuse::apply(all_backends)]
    fn test_delete_not_exists(mut kv: Box<dyn UserKvStorage>) {
        // Arrange

        // Act
        assert!(!kv.exists("nonexistant_user").unwrap());
        kv.delete("nonexistant_user").unwrap();
        assert!(!kv.exists("nonexistant_user").unwrap());
    }
}
