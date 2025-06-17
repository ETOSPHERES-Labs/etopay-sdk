use super::newtypes::{EncryptedPassword, EncryptionSalt};
use crate::{
    types::viviswap::ViviswapState,
    wallet_manager::{WalletManager, WalletManagerImpl},
};
use etopay_wallet::{
    MnemonicDerivationOption,
    types::{WalletTxInfo, WalletTxInfoVersioned},
};
use serde::{Deserialize, Serialize};

/// Struct for storing a user in the database
#[derive(Debug, Deserialize, Serialize, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct UserEntity {
    /// User ID for backend (remove or use for telemetry?)
    pub user_id: Option<String>,
    /// Username for DB
    pub username: String,
    /// Encrypted Password
    pub encrypted_password: Option<EncryptedPassword>,
    /// Salt
    pub salt: EncryptionSalt,
    /// User KYC status
    pub is_kyc_verified: bool,
    /// User KYC Type
    pub kyc_type: KycType,
    /// User Viviswap state
    pub viviswap_state: Option<ViviswapState>,

    /// The local share from the SSS scheme, stored as a string (same as in the backend)
    pub local_share: Option<String>,

    /// User wallet transactions
    pub wallet_transactions: Vec<WalletTxInfo>,

    /// User wallet transactions (versioned)
    #[serde(default)]
    pub wallet_transactions_versioned: Vec<WalletTxInfoVersioned>,
}

/// Struct to manage the state of the currently active (initialized) user
#[derive(Debug)]
pub struct ActiveUser {
    /// Username
    pub username: String,

    /// The user's wallet manager that can create a WalletUser instance from shares.
    pub wallet_manager: Box<dyn WalletManager + Send + Sync + 'static>,

    /// The currently active [`MnemonicDerivationOption`]
    pub mnemonic_derivation_options: MnemonicDerivationOption,
}

impl From<UserEntity> for ActiveUser {
    fn from(entity: UserEntity) -> Self {
        ActiveUser {
            wallet_manager: Box::new(WalletManagerImpl::new(&entity.username)),
            username: entity.username,
            mnemonic_derivation_options: Default::default(),
        }
    }
}

/// Represents which kyc method the user uses
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub enum KycType {
    /// Kyc process not selected
    Undefined,

    /// User use postident for kyc
    #[cfg(feature = "postident")]
    Postident,

    /// User use viviswap for kyc
    #[cfg(feature = "viviswap-kyc")]
    Viviswap,
}
