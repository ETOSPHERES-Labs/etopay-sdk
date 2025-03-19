use super::error::Result;
use super::UserKvStorageError;
use crate::types::users::UserEntity;
use base64::prelude::*;
use web_sys::{js_sys, wasm_bindgen::JsValue};

const STORAGE_KEY_PREFIX: &str = "etopay.local.user";

/// An implementation of [`super::UserKvStorage`] that uses the browsers local storage, rmp_serde
/// and base64-encoding to store user entities.
pub struct BrowserLocalStorage {}

impl BrowserLocalStorage {
    pub fn new() -> Self {
        Self {}
    }

    fn get_storage(&self) -> Result<web_sys::Storage> {
        let window_obj = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("window"))
            .map_err(|e| UserKvStorageError::Storage(format!("no window object found: {e:?}")))?;

        let local_storage_obj = js_sys::Reflect::get(&window_obj, &JsValue::from_str("localStorage"))
            .map_err(|e| UserKvStorageError::Storage(format!("no window.localStorage object found: {e:?}")))?;

        let storage = web_sys::Storage::try_from(local_storage_obj)
            .map_err(|e| UserKvStorageError::Storage(format!("window.localStorage should be web_sys::Storage: {e}")))?;
        Ok(storage)
    }

    /// Checks if storage is available and can be written to
    pub fn is_available(&self) -> bool {
        const STORAGE_TEST_KEY: &str = "etopay.test.available";
        const STORAGE_TEST_PAYLOAD: &str = "test_payload";

        let Ok(storage) = self.get_storage() else { return false };

        if storage.set_item(STORAGE_TEST_KEY, STORAGE_TEST_PAYLOAD).is_err() {
            return false;
        }

        let Ok(Some(value)) = storage.get_item(STORAGE_TEST_KEY) else {
            return false;
        };

        let _ = storage.remove_item(STORAGE_TEST_KEY);

        value == STORAGE_TEST_PAYLOAD
    }

    fn storage_user_key(&self, username: &str) -> String {
        format!("{STORAGE_KEY_PREFIX}.{username}")
    }
}

impl From<base64::DecodeError> for UserKvStorageError {
    fn from(value: base64::DecodeError) -> Self {
        UserKvStorageError::Storage(format!("base64::DecodeError: {:#?}", value))
    }
}

impl super::UserKvStorage for BrowserLocalStorage {
    fn get(&self, username: &str) -> Result<UserEntity> {
        if let Some(value) = self
            .get_storage()?
            .get_item(&self.storage_user_key(username))
            .map_err(|e| UserKvStorageError::Storage(format!("Could not get storage key {username}: {e:#?}")))?
        {
            let bytes = BASE64_STANDARD.decode(value)?;
            let user: UserEntity = rmp_serde::from_slice(&bytes)?;
            Ok(user)
        } else {
            Err(UserKvStorageError::UserNotFound {
                username: username.to_string(),
            })
        }
    }

    fn delete(&mut self, username: &str) -> Result<()> {
        self.get_storage()?
            .remove_item(&self.storage_user_key(username))
            .map_err(|e| UserKvStorageError::Storage(format!("Could not remove storage key {username}: {e:#?} ")))
    }

    fn exists(&self, username: &str) -> Result<bool> {
        Ok(self
            .get_storage()?
            .get_item(&self.storage_user_key(username))
            .map_err(|e| UserKvStorageError::Storage(format!("Could not get storage key {username}: {e:#?}")))?
            .is_some())
    }

    fn set(&mut self, username: &str, value: &UserEntity) -> Result<()> {
        let storage = self.get_storage()?;

        let bytes = rmp_serde::to_vec(value)?;
        let text = BASE64_STANDARD.encode(bytes);

        storage
            .set_item(&self.storage_user_key(username), &text)
            .map_err(|e| UserKvStorageError::Storage(format!("Could not set storage key {username}: {e:#?}")))?;

        Ok(())
    }
}
