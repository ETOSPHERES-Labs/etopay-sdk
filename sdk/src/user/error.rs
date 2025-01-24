/// A [`core::result::Result`] with [`UserKvStorageError`] as its error variant.
pub type Result<T> = core::result::Result<T, UserKvStorageError>;

#[derive(thiserror::Error, Debug)]
pub enum UserKvStorageError {
    /// The user already exists in the KV storage.
    #[error("User already exists: {username}")]
    UserAlreadyExists { username: String },

    /// The user is not found in the KV storage,
    #[error("User not found: {username}")]
    UserNotFound { username: String },

    /// An internal storage error happened (backend specific)
    #[error("Internal storage error: {0}")]
    Storage(String),
}
