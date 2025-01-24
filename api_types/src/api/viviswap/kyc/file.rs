use base64::prelude::*;
use serde::{Deserialize, Serialize};

pub type FileError = base64::DecodeError;

/// A file which can be safely sent across the HTTP API by encoding the file
/// bytes as base64.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct File {
    pub data: String,
    pub filename: String,
}

impl File {
    /// Construct a [`File`] from a set of bytes
    pub fn from_bytes(data: &[u8], filename: &str) -> Self {
        Self {
            data: BASE64_STANDARD.encode(data),
            filename: filename.to_string(),
        }
    }

    /// Try to construct a [`File`] from an existing base64-encoded string.
    /// This will return an error if the provided string is not correctly encoded.
    pub fn try_from_base64(data: &str, filename: &str) -> Result<Self, FileError> {
        // make sure data is valid by first trying to convert to bytes, if error we return an error, otherwise we can use the string
        BASE64_STANDARD.decode(data)?;

        Ok(Self {
            data: data.to_owned(),
            filename: filename.to_string(),
        })
    }

    /// Get the filename of this file
    pub fn filename(&self) -> &str {
        &self.filename
    }

    /// Get the raw data bytes of this file
    pub fn bytes(&self) -> Result<Vec<u8>, FileError> {
        BASE64_STANDARD.decode(&self.data)
    }
}
