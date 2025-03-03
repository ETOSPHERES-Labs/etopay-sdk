//! This module provides the repository implementation for managing user state.
//! It defines the `UserRepo` trait and implementations using a file-backed database or
//! an in-memory non-persistent database.
//!
//! The `UserRepo` trait defines methods for creating, updating, deleting, and retrieving user entities,
//! as well as methods for setting user state, password, KYC state, KYC type, and viviswap KYC state.
//!

pub mod error;
pub mod repository;

use crate::{
    share::Share,
    types::{
        newtypes::EncryptedPassword,
        transactions::WalletTxInfo,
        users::{KycType, UserEntity},
        viviswap::{ViviswapVerificationStatus, ViviswapVerificationStep},
    },
};
use api_types::api::account::Customer;
use error::{Result, UserKvStorageError};

/// Storage abstraction of [`UserEntity`] objects as a simple Key-Value storage
#[cfg_attr(test, mockall::automock)]
pub trait UserKvStorage {
    /// Get a value by key. Returns Error if key does not exist.
    fn get(&self, username: &str) -> Result<UserEntity>;

    /// Remove a key and the associated value. No error if the key does not exist.
    fn delete(&mut self, username: &str) -> Result<()>;

    /// Check if a key exists.
    fn exists(&self, username: &str) -> Result<bool>;

    /// Associate a key with a value. This will overwrite any previous value.
    fn set(&mut self, username: &str, value: &UserEntity) -> Result<()>;
}

/// Represents the storage and loading of different users in a repository. This could be
/// either a file on disk or a completely in-memory storage.
#[cfg_attr(test, mockall::automock)]
pub trait UserRepo {
    /// Create a new user.
    ///
    /// # Arguments
    ///
    /// * `user` - The user entity to create.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is created successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::UserAlreadyExists` if the user already exists in the database.
    fn create(&mut self, user: &UserEntity) -> Result<()>;

    ///  Update user
    ///
    /// # Arguments
    ///
    /// * `user` - The user entity to update.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is updated successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::UserNotFound` if the user does not exist in the database.
    fn update(&mut self, user: &UserEntity) -> Result<()>;

    /// Delete user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to delete.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the user is deleted successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::UserNotFound` if the user does not exist in the database.
    fn delete(&mut self, username: &str) -> Result<()>;

    /// Get an user
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user to get.
    ///
    /// # Returns
    ///
    /// Returns the `UserEntity` if the user is found, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn get(&self, username: &str) -> Result<UserEntity>;

    /// Set the wallet password for a user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `password` - The password to set for the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the password is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_wallet_password(&mut self, username: &str, password: EncryptedPassword) -> Result<()>;

    /// Set user kyc state.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `is_verified` - A boolean indicating whether the KYC is verified or not.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the KYC state is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_kyc_state(&mut self, username: &str, is_verified: bool) -> Result<()>;

    /// Set KYC type.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `kyc_type` - The KYC type to set for the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the KYC type is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_kyc_type(&mut self, username: &str, kyc_type: KycType) -> Result<()>;

    /// Set the Viviswap KYC state for a user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `verification_status` - The verification status to set for the user.
    /// * `monthly_limit_eur` - The monthly limit in EUR to set for the user.
    /// * `next_verification_step` - The next verification step to set for the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the Viviswap KYC state is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_viviswap_kyc_state(
        &mut self,
        username: &str,
        verification_status: ViviswapVerificationStatus,
        monthly_limit_eur: f32,
        next_verification_step: ViviswapVerificationStep,
    ) -> Result<()>;

    /// Set the customer details for a user.
    ///
    /// # Arguments
    ///
    /// * `customer` - The customer details to set for the user.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the customer details are set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_customer_details(&mut self, customer: Customer) -> Result<()>;

    /// Set wallet transactions.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `transaction` - List of transaction
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the wallet transaction are set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error retrieving the user from the database.
    fn set_wallet_transactions(&mut self, username: &str, transaction: Vec<WalletTxInfo>) -> Result<()>;

    /// Set the local share for a user.
    ///
    /// # Arguments
    ///
    /// * `username` - The username of the user.
    /// * `share` - The local share to store, or [`None`] if the local share should be cleared.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the local share is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// Returns an `Error::KVError` if there is an error storing the share in the database.
    #[allow(clippy::needless_lifetimes)] // the explicit lifetime 'a is needed for mockall::automock to work correctly
    fn set_local_share<'a>(&mut self, username: &str, share: Option<&'a Share>) -> Result<()>;
}

/// An implementation of [`UserKvStorage`] using a jammdb file-based database.
#[cfg(feature = "jammdb_repo")]
pub mod file_storage;

/// An implementation of [`UserKvStorage`] that uses the browsers local storage.
#[cfg(target_arch = "wasm32")]
pub mod web_storage;

/// An implementation of [`UserKvStorage`] that uses a non-persistent in-memory storage.
pub mod memory_storage;

// implementations shared by file and web storage

#[cfg(any(feature = "jammdb_repo", target_arch = "wasm32"))]
impl From<rmp_serde::decode::Error> for UserKvStorageError {
    fn from(value: rmp_serde::decode::Error) -> Self {
        UserKvStorageError::Storage(format!("rmp_serde::decode::Error: {:#?}", value))
    }
}

#[cfg(any(feature = "jammdb_repo", target_arch = "wasm32"))]
impl From<rmp_serde::encode::Error> for UserKvStorageError {
    fn from(value: rmp_serde::encode::Error) -> Self {
        UserKvStorageError::Storage(format!("rmp_serde::encode::Error: {:#?}", value))
    }
}
