//! Contains logic for interfacing with the IOTA Rebased Network.
//! This deliberately exposes a minimal set of types / interfaces so that it can easily be
//! moved to a separate crate if we want to in the future. It should not import or use anything
//! from the rest of the sdk crate!
//!

#![allow(dead_code, reason = "Not all copied methods are needed but kept for the future")]

mod bigint;
mod client;
mod encoding;
mod error;
mod hash;
// mod iota_json;
mod keystore;
mod mowe;
mod rpc;
mod serde;
mod traits;
mod types;

mod v2;
pub use v2::*;
pub mod crypto;

pub use mowe::*;
pub use rpc::*;
pub use types::*;

pub use client::RpcClient;
pub use error::RebasedError;
pub use keystore::InMemKeystore;
