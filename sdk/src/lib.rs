#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

// pub modules accessible by bindings
pub mod core;
pub mod types;

// expose error and result types in the crate root
pub mod error;
pub use error::{Error, Result};

// internal modules
mod backend;
mod user;
mod wallet;
pub use wallet::error::{ErrorKind, WalletError};

#[cfg(not(target_arch = "wasm32"))]
mod logger;

pub use wallet::*;

/// mod used for sdk unit-test utilities
#[cfg(test)]
pub(crate) mod testing_utils;

/// Exported secrecy crate to use in the bindings
pub use secrecy;

/// Helper macro for bindings to return an error if the feature is not enabled.
/// Produces an Err(String) variant if the feature is not enabled, and the value of the
/// body if the feature is enabled.
#[macro_export]
macro_rules! require_feature {
    // this arm adds a default String error message
    ($feature: literal, $body: block) => {{
        $crate::require_feature!(
            $feature,
            $body,
            format!("SDK is not compiled with feature `{}` enabled", $feature)
        )
    }};
    // this one allows specifying the error value manually
    ($feature: literal, $body: block, $error: expr) => {{
        #[cfg(not(feature = $feature))]
        {
            Err($error)
        }

        #[cfg(feature = $feature)]
        $body
    }};
}

use shadow_rs::shadow;
shadow!(build);
