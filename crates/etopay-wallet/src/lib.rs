mod error;
mod rebased;
mod wallet;
mod wallet_evm;
mod wallet_rebased;

pub mod migrations;
pub mod types;

pub use error::{Result, WalletError};
pub use migrations::*;
pub use wallet::*;
pub use wallet_evm::{WalletImplEvm, WalletImplEvmErc20};
pub use wallet_rebased::WalletImplIotaRebased;

/// Re-export the bip39 crate so that our users can create [`bip39::Mnemonic`]s
pub use bip39;
