//! Managing and using the wallet
//!
//! Provides an abstraction for wallet setup via different methods
//! and managing wallet for different application specific activities
//!

/// Wallet manager
pub mod wallet_manager;

/// wallet user definition
#[allow(clippy::module_inception)]
pub(crate) mod wallet;

/// wallet user for Stardust protocol
pub(crate) mod wallet_stardust;

/// wallet user for EVM
pub(crate) mod wallet_evm;

pub(crate) mod wallet_rebased;

/// Module containing code related to the SSS secret sharing scheme
pub mod share;

/// Module containing code related to the KDBX file format
pub mod kdbx;

/// Errors related to sdk wallet
pub mod error;
