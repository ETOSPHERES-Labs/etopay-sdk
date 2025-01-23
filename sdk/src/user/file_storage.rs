//! This module provides the implementation of the `UserRepo` using a file-based database.
//! This is mainly used on systems that have access to the file system as it provides a persistent
//! storage mechanism.

use super::error::{Result, UserKvStorageError};
use crate::types::users::UserEntity;
use jammdb::DB;
use log::warn;
use std::path::Path;

#[doc = r"The default name of the local memory mapped DB"]
const SHARED_DB_NAME: &str = "sdk-user.db";
#[doc = r"The default name of the bucket of the memory mapped DB"]
const DB_BUCKET: &str = "users";

/// Implementation of [`super::UserKvStorage`] using a [`jammdb`] file-based database.
pub struct FileUserStorage {
    db: DB,
}

impl FileUserStorage {
    /// Initialize a new instance and create a db file
    pub fn new(path_prefix: &Path) -> Result<Self> {
        let path = path_prefix.join(SHARED_DB_NAME);
        warn!("Attempting to create user DB in path: {path:?}");

        Ok(Self { db: DB::open(path)? })
    }
}

impl From<jammdb::Error> for UserKvStorageError {
    fn from(value: jammdb::Error) -> Self {
        UserKvStorageError::Storage(format!("jammdb::Error: {:#?}", value))
    }
}

impl super::UserKvStorage for FileUserStorage {
    fn get(&self, username: &str) -> Result<UserEntity> {
        let tx = self.db.tx(true)?;

        // get the bucket we created in the last transaction
        let users_bucket = tx.get_or_create_bucket(DB_BUCKET)?;

        let kv = users_bucket
            .get_kv(username)
            .ok_or_else(|| UserKvStorageError::UserNotFound {
                username: username.to_owned(),
            })?;

        let user: UserEntity = rmp_serde::from_slice(kv.value())?;

        Ok(user)
    }

    fn delete(&mut self, username: &str) -> Result<()> {
        let tx = self.db.tx(true)?;
        let users_bucket = tx.get_or_create_bucket(DB_BUCKET)?;

        // delete and ignore error if key does not exist, but propagate other errors
        match users_bucket.delete(username) {
            Err(jammdb::Error::KeyValueMissing) => {}
            Err(e) => return Err(e.into()),
            Ok(_) => {}
        }

        // commit the changes so they are saved to disk
        tx.commit()?;
        Ok(())
    }

    fn exists(&self, username: &str) -> Result<bool> {
        let tx = self.db.tx(true)?;
        let users_bucket = tx.get_or_create_bucket(DB_BUCKET)?;
        Ok(users_bucket.get_kv(username).is_some())
    }

    fn set(&mut self, username: &str, value: &UserEntity) -> Result<()> {
        let tx = self.db.tx(true)?;
        let users_bucket = tx.get_or_create_bucket(DB_BUCKET)?;

        // serialize struct to bytes and store in bucket
        let user_bytes = rmp_serde::to_vec(&value)?;
        users_bucket.put(username, user_bytes)?;

        // commit the changes so they are saved to disk
        tx.commit()?;

        Ok(())
    }
}
