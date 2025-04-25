//! Contains logic for interfacing with the IOTA Rebased Network.
//! This deliberately exposes a minimal set of types / interfaces so that it can easily be
//! moved to a separate crate if we want to in the future. It should not import or use anything
//! from the rest of the sdk crate!
//!

#![allow(clippy::expect_used, clippy::unwrap_used)] // used in some serialize/deserialize locations
#![allow(dead_code)] // TEMP

mod bigint;
mod client;
mod keystore;
mod rpc;
mod serde;
mod types;

pub use rpc::*;
pub use types::*;


pub use keystore::InMemKeystore;

pub use client::RpcClient;
