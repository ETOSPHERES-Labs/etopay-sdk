//! This module provides methods for initializing, verifying, deleting, and creating wallets, as well as
//! migrating wallets from mnemonic or backup, creating backups, and verifying PINs.
//!
//! It also includes various helper functions and imports required for the wallet functionality.
use super::Sdk;
use crate::{
    backend::dlt::put_user_address,
    error::Result,
    tx_version::VersionedWalletTransaction,
    types::newtypes::{EncryptionPin, EncryptionSalt, PlainPassword},
    wallet::error::{ErrorKind, WalletError},
};
use etopay_wallet::{
    MnemonicDerivationOption,
    types::{CryptoAmount, WalletTransaction, WalletTxInfoList, WalletTxStatus},
};

use log::{debug, info, warn};

impl Sdk {
    /// Create and store a wallet from a new random mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    ///
    /// # Returns
    ///
    /// The new random mnemonic.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_new_mnemonic(&mut self, pin: &EncryptionPin) -> Result<String> {
        info!("Creating a new wallet from random mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let mnemonic = active_user
            .wallet_manager
            .create_wallet_from_new_mnemonic(config, &self.access_token, repo, pin)
            .await?;
        Ok(mnemonic)
    }

    /// Create and store a wallet from an existing mnemonic
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to use for the wallet.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_existing_mnemonic(&mut self, pin: &EncryptionPin, mnemonic: &str) -> Result<()> {
        info!("Creating a new wallet from existing mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .create_wallet_from_existing_mnemonic(config, &self.access_token, repo, pin, mnemonic)
            .await?;
        Ok(())
    }

    /// Create and store a wallet from an existing kdbx backup file
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup` - The bytes representing the backup file.
    /// * `backup_password` - The password used when creating the backup file.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_from_backup(
        &mut self,
        pin: &EncryptionPin,
        backup: &[u8],
        backup_password: &PlainPassword,
    ) -> Result<()> {
        info!("Creating a new wallet from backup");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .create_wallet_from_backup(config, &self.access_token, repo, pin, backup, backup_password)
            .await?;
        Ok(())
    }

    /// Create a kdbx wallet backup from an existing wallet.
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `backup_password` - The password to use when creating the backup file.
    ///
    /// # Returns
    ///
    /// The bytes of the kdbx backup file.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn create_wallet_backup(
        &mut self,
        pin: &EncryptionPin,
        backup_password: &PlainPassword,
    ) -> Result<Vec<u8>> {
        info!("Creating wallet backup");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let backup = active_user
            .wallet_manager
            .create_wallet_backup(config, &self.access_token, repo, pin, backup_password)
            .await?;
        Ok(backup)
    }

    /// Verify the mnemonic by checking if the mnemonic is the same as the one in the shares
    ///
    /// # Arguments
    ///
    /// * `pin` - The PIN for the wallet.
    /// * `mnemonic` - The mnemonic to verify.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the mnemonic is successfully verified, otherwise returns `Ok(false)`,
    /// or an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn verify_mnemonic(&mut self, pin: &EncryptionPin, mnemonic: &str) -> Result<bool> {
        info!("Verifying mnemonic");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        let is_verified = active_user
            .wallet_manager
            .check_mnemonic(config, &self.access_token, repo, pin, mnemonic)
            .await?;
        Ok(is_verified)
    }

    /// Delete the currently active wallet
    ///
    /// Deletes the currently active wallet, potentially resulting in loss of funds if the mnemonic or wallet is not backed up.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the wallet is successfully deleted, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn delete_wallet(&mut self, pin: &EncryptionPin) -> Result<()> {
        warn!("Deleting wallet for user. Potential loss of funds if mnemonic/wallet is not backed up!");

        self.verify_pin(pin).await?;

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

        active_user
            .wallet_manager
            .delete_wallet(config, &self.access_token, repo)
            .await?;

        Ok(())
    }

    /// Verify pin
    ///
    /// Verifies the pin for the wallet.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to verify.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the pin is verified successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    /// * [`WalletError::WrongPinOrPassword`] - If the pin or password is incorrect.
    pub async fn verify_pin(&self, pin: &EncryptionPin) -> Result<()> {
        info!("Verifying wallet pin");
        let Some(repo) = &self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let user = repo.get(username)?;

        // Ensure encrypted password exists in user
        let Some(encrypted_password) = user.encrypted_password else {
            return Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))?;
        };

        // Decrypt the password using the provided PIN
        if encrypted_password.decrypt(pin, &user.salt).is_err() {
            return Err(WalletError::WrongPinOrPassword)?;
        }
        Ok(())
    }

    /// Reset pin
    ///
    /// Resets the pin for the wallet using the provided password and new pin.
    ///
    /// # Arguments
    ///
    /// * `old_pin` - The old wallet pin.
    /// * `new_pin` - The new pin to set for the wallet.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the pin is changed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    /// * [`WalletError::WrongPinOrPassword`] - If the pin or password is incorrect.
    pub async fn change_pin(&mut self, old_pin: &EncryptionPin, new_pin: &EncryptionPin) -> Result<()> {
        info!("Resetting pin with password");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let username = &active_user.username;
        let mut user = repo.get(username)?;

        let Some(encrypted_password) = user.encrypted_password else {
            return Err(WalletError::WalletNotInitialized(ErrorKind::MissingPassword))?;
        };

        // decrypt the password
        let password = encrypted_password.decrypt(old_pin, &user.salt)?;

        // Set new pin and encrypted password
        let salt = EncryptionSalt::generate();
        let encrypted_password = password.encrypt(new_pin, &salt)?;

        // Update user
        user.salt = salt;
        user.encrypted_password = Some(encrypted_password);
        repo.update(&user)?;

        Ok(())
    }

    /// Set the password to use for wallet operations. If the password was already set, this changes it.
    ///
    /// # Arguments
    ///
    /// * `pin` - The pin to encrypt the password with.
    /// * `new_password` - The new password to set for the wallet.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the password is set successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn set_wallet_password(&mut self, pin: &EncryptionPin, new_password: &PlainPassword) -> Result<()> {
        info!("Setting password");

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let mut user = repo.get(&active_user.username)?;

        // if password already exists, return an error!
        if let Some(encrypted_password) = user.encrypted_password {
            info!("Password exists, changing password");

            // verify that the pin is correct by decrypting the password using the provided PIN
            if encrypted_password.decrypt(pin, &user.salt).is_err() {
                return Err(WalletError::WrongPinOrPassword)?;
            }

            let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;

            active_user
                .wallet_manager
                .change_wallet_password(config, &self.access_token, repo, pin, new_password)
                .await?;
        } else {
            // Set new pin and encrypted password
            let salt = EncryptionSalt::generate();
            let encrypted_password = new_password.encrypt(pin, &salt)?;

            // Update user
            user.salt = salt;
            user.encrypted_password = Some(encrypted_password);
            repo.update(&user)?;
        }

        Ok(())
    }

    /// Check if the password to use for wallet operations is set. If this returns `false`,
    /// the password should be set with [`set_wallet_password`], otherwise you need to use
    /// [`change_password`] to change it.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the password is set successfully, otherwise returns `Ok(false)`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn is_wallet_password_set(&self) -> Result<bool> {
        info!("Checking if password is set");

        let Some(repo) = &self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let user = repo.get(&active_user.username)?;

        Ok(user.encrypted_password.is_some())
    }

    /// Generates a new receiver address (based on selected currency in the config) for the wallet.
    ///
    /// # Returns
    ///
    /// Returns the generated address as a `String` if successful, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserRepoNotInitialized`] - If there is an error initializing the repository.
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`crate::Error::MissingConfig`] - If the sdk config is missing.
    pub async fn generate_new_address(&mut self, pin: &EncryptionPin) -> Result<String> {
        info!("Generating new wallet address");
        self.verify_pin(pin).await?;
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let network = self.active_network.as_ref().ok_or(crate::Error::MissingNetwork)?;
        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let wallet = active_user
            .wallet_manager
            .try_get(
                config,
                &self.access_token,
                repo,
                network,
                pin,
                &active_user.mnemonic_derivation_options,
            )
            .await?;

        let address = wallet.get_address().await?;

        // if there is an access token, push the generated address to the backend
        if let Some(access_token) = self.access_token.as_ref() {
            if network.can_do_purchases {
                put_user_address(config, access_token, &network.key, &address).await?;
            }
        }
        debug!("Generated address: {address}");
        Ok(address)
    }

    /// Get the balance of the user
    ///
    /// Fetches the balance of the user from the wallet.
    ///
    /// # Returns
    ///
    /// Returns the balance as a `f64` if successful, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_balance(&mut self, pin: &EncryptionPin) -> Result<CryptoAmount> {
        info!("Fetching balance");
        self.verify_pin(pin).await?;
        let wallet = self.try_get_active_user_wallet(pin).await?;
        let balance = wallet.get_balance().await?;
        debug!("Balance: {balance:?}");
        Ok(balance)
    }

    /// wallet transaction list
    ///
    /// Returns paginated list of wallet transaction list.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting index of transactions to fetch.
    /// * `limit` - The number of transactions per page.
    ///
    /// # Returns
    ///
    /// Returns a `WalletTxInfoList` containing paginated history of wallet transactions if the outputs are claimed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_wallet_tx_list(
        &mut self,
        pin: &EncryptionPin,
        start: usize,
        limit: usize,
    ) -> Result<WalletTxInfoList> {
        info!("Wallet getting list of transactions");
        self.verify_pin(pin).await?;

        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let network = self.active_network.as_ref().ok_or(crate::Error::MissingNetwork)?;
        let config = self.config.as_mut().ok_or(crate::Error::MissingConfig)?;
        let wallet = active_user
            .wallet_manager
            .try_get(
                config,
                &self.access_token,
                repo,
                network,
                pin,
                &active_user.mnemonic_derivation_options,
            )
            .await?;

        let user = repo.get(active_user.username.as_str())?;

        // Retrieve the transaction list from the wallet,
        // 1) fetch and add new (untracked) transactions from the network,
        // 2) migrate transactions to the latest version if necessary,
        // 3) confirm pending transactions,
        // 4) and save the updated list back to the wallet.
        let mut wallet_transactions = user.wallet_transactions_versioned;

        // 1) fetch and add new (untracked) transactions from the network
        match wallet.get_wallet_tx_list(start, limit).await {
            Ok(transaction_hashes) => {
                // go through and get the details for any new hashes
                log::debug!("Digests: {:#?}", transaction_hashes);
                for hash in transaction_hashes {
                    // check if transaction is already in the list (not very efficient to do a linear search, but good enough for now)
                    // check both the transaction hash and the network key, as hash collisions can occur across different blockchain networks
                    if wallet_transactions
                        .iter()
                        .any(|t| t.transaction_hash() == hash && t.network_key() == network.key)
                    {
                        continue;
                    }

                    log::debug!("Getting details for new transaction with hash {hash}");

                    // not included, we should add it!
                    match wallet.get_wallet_tx(&hash).await {
                        Err(e) => log::warn!("Could not get transaction details for {hash}: {e}"),
                        Ok(details) => wallet_transactions.push(VersionedWalletTransaction::from(details)),
                    }
                }
            }
            // do nothing if feature is not supported
            Err(etopay_wallet::WalletError::WalletFeatureNotImplemented) => {}
            Err(e) => return Err(e.into()),
        };

        wallet_transactions.sort_by_key(|b| std::cmp::Reverse(b.date()));
        let mut wallet_tx_list = Vec::new();

        for t in wallet_transactions
            .iter_mut()
            .filter(|tx| tx.network_key() == network.key)
            .skip(start)
            .take(limit)
        {
            // 2) migrate transactions to the latest version if necessary
            if let VersionedWalletTransaction::V1(v1) = t {
                if let Ok(details) = wallet.get_wallet_tx(&v1.transaction_hash).await {
                    *t = VersionedWalletTransaction::from(details);
                    wallet_tx_list.push(WalletTransaction::from(t.clone()));
                    continue;
                }
            }

            // 3) confirm pending transactions
            if t.status() == WalletTxStatus::Pending {
                if let Ok(details) = wallet.get_wallet_tx(t.transaction_hash()).await {
                    *t = VersionedWalletTransaction::V2(details);
                    wallet_tx_list.push(WalletTransaction::from(t.clone()));
                    continue;
                }
            }

            wallet_tx_list.push(WalletTransaction::from(t.clone()));
        }

        // 4) and save the updated list back to the wallet.
        let _ = repo.set_wallet_transactions(&user.username, wallet_transactions);

        Ok(WalletTxInfoList {
            transactions: wallet_tx_list,
        })
    }

    /// wallet transaction
    ///
    /// Returns the wallet transaction details.
    ///
    /// # Arguments
    ///
    /// * `tx_id` - The transaction id of particular transaction.
    ///
    /// # Returns
    ///
    /// Returns `WalletTransaction` detailed report of particular wallet transaction if the outputs are claimed successfully, otherwise returns an `Error`.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    /// * [`WalletError::WalletNotInitialized`] - If there is an error initializing the wallet.
    pub async fn get_wallet_tx(&mut self, pin: &EncryptionPin, tx_id: &str) -> Result<WalletTransaction> {
        info!("Wallet getting details of particular transactions");
        self.verify_pin(pin).await?;
        let wallet = self.try_get_active_user_wallet(pin).await?;
        let wallet_tx = wallet.get_wallet_tx(tx_id).await?;
        Ok(wallet_tx)
    }

    /// Set wallet mnemonic derivation options
    ///
    /// # Arguments
    ///
    /// * `account` - The account to use.
    /// * `index` - The index to use.
    ///
    /// # Errors
    ///
    /// * [`crate::Error::UserNotInitialized`] - If there is an error initializing the user.
    pub async fn set_wallet_derivation_options(&mut self, account: u32, index: u32) -> Result<()> {
        let options = MnemonicDerivationOption { account, index };

        info!("Setting wallet mnemonic derivation options: {options:?}");

        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        active_user.mnemonic_derivation_options = options;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{
        ADDRESS, AUTH_PROVIDER, ENCRYPTED_WALLET_PASSWORD, ETH_NETWORK_KEY, HEADER_X_APP_NAME, IOTA_NETWORK_KEY,
        MNEMONIC, PIN, SALT, TOKEN, TX_INDEX, USERNAME, WALLET_PASSWORD, example_api_networks, example_get_user,
        example_versioned_wallet_transaction, set_config,
    };
    use crate::types::users::UserEntity;
    use crate::{
        core::Sdk,
        types::users::KycType,
        user::MockUserRepo,
        wallet_manager::{MockWalletManager, WalletBorrow},
    };
    use api_types::api::dlt::SetUserAddressRequest;
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use chrono::{DateTime, TimeZone, Utc};
    use etopay_wallet::MockWalletUser;
    use etopay_wallet::types::{WalletTransaction, WalletTxStatus};
    use mockall::predicate::eq;
    use mockito::Matcher;
    use rstest::rstest;
    use rust_decimal_macros::dec;
    use std::sync::LazyLock;

    const BACKUP: &[u8] = &[42, 77, 15, 203, 89, 123, 34, 56, 178, 90, 210, 33, 47, 192, 1, 17];

    #[rstest]
    #[case::success(Ok(MNEMONIC.to_string()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_new_mnemonic(#[case] expected: Result<String>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_new_mnemonic()
                    .once()
                    .returning(|_, _, _, _| Ok(MNEMONIC.to_string()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_new_mnemonic(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_existing_mnemonic(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_existing_mnemonic()
                    .once()
                    .returning(|_, _, _, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_existing_mnemonic(&PIN, MNEMONIC).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(BACKUP.to_vec()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_backup(#[case] expected: Result<Vec<u8>>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_backup()
                    .once()
                    .returning(|_, _, _, _, _| Ok(BACKUP.to_vec()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_backup(&PIN, &WALLET_PASSWORD).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_create_wallet_from_backup(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_create_wallet_from_backup()
                    .once()
                    .returning(|_, _, _, _, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_wallet_from_backup(&PIN, BACKUP, &WALLET_PASSWORD).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(true))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_verify_mnemonic(#[case] expected: Result<bool>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_check_mnemonic()
                    .once()
                    .returning(|_, _, _, _, _| Ok(true));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.verify_mnemonic(&PIN, MNEMONIC).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_delete_wallet(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let pin = EncryptionPin::try_from_string("123456").unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 2, KycType::Undefined);
                mock_user_repo.expect_update().once().returning(|_| Ok(()));

                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_delete_wallet()
                    .once()
                    .returning(|_, _, _| Ok(()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });

                let new_pin = EncryptionPin::try_from_string("123456").unwrap();
                sdk.change_pin(&pin, &new_pin).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.delete_wallet(&PIN).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::wallet_not_initialized(Err(crate::Error::Wallet(WalletError::WalletNotInitialized(
        ErrorKind::MissingPassword
    ))))]
    #[tokio::test]
    async fn test_verify_pin(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.verify_pin(&PIN).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::wallet_not_initialized(Err(crate::Error::Wallet(WalletError::WalletNotInitialized(
        ErrorKind::MissingPassword
    ))))]
    #[tokio::test]
    async fn test_change_pin(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo.expect_update().once().returning(|_| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let new_pin: LazyLock<EncryptionPin> = LazyLock::new(|| EncryptionPin::try_from_string("432154").unwrap());
        let response = sdk.change_pin(&PIN, &new_pin).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_set_wallet_password(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = MockUserRepo::new();
                mock_user_repo.expect_get().times(1).returning(move |r1| {
                    assert_eq!(r1, USERNAME);
                    Ok(UserEntity {
                        user_id: None,
                        username: USERNAME.into(),
                        encrypted_password: None,
                        salt: SALT.into(),
                        is_kyc_verified: false,
                        kyc_type: KycType::Undefined,
                        viviswap_state: Option::None,
                        local_share: None,
                        wallet_transactions: Vec::new(),
                        wallet_transactions_versioned: Vec::new(),
                    })
                });
                mock_user_repo.expect_update().once().returning(|_| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                    mnemonic_derivation_options: Default::default(),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.set_wallet_password(&PIN, &WALLET_PASSWORD).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(ADDRESS.to_string()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_generate_new_address(#[case] expected: Result<String>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_address()
                        .once()
                        .returning(|| Ok(ADDRESS.to_string()));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
                sdk.access_token = Some(TOKEN.clone());
                sdk.set_networks(example_api_networks());
                sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();

                let mock_request = SetUserAddressRequest {
                    address: ADDRESS.into(),
                };
                let body = serde_json::to_string(&mock_request).unwrap();

                mock_server = Some(
                    srv.mock("PUT", "/api/user/address")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .match_header("content-type", "application/json")
                        .match_query(Matcher::Exact("network_key=IOTA".to_string()))
                        .match_body(Matcher::Exact(body))
                        .with_status(201)
                        .expect(1)
                        .with_header("content-type", "application/json")
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.generate_new_address(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }

    #[rstest]
    // SAFETY: we know that this value is not negative
    #[case::success(Ok(unsafe { CryptoAmount::new_unchecked(dec!(25.0)) }))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_balance(#[case] expected: Result<CryptoAmount>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_balance()
                        .once()
                        // SAFETY: we know that this value is not negative
                        .returning(|| Ok(unsafe { CryptoAmount::new_unchecked(dec!(25.0)) }));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.get_balance(&PIN).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(WalletTransaction::from(example_versioned_wallet_transaction())))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_wallet_tx(#[case] expected: Result<WalletTransaction>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_wallet_tx()
                        .once()
                        .returning(|_| Ok(WalletTransaction::from(example_versioned_wallet_transaction())));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.get_wallet_tx(&PIN, TX_INDEX).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(WalletTxInfoList { transactions: vec![]}))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_wallet_tx_list(#[case] expected: Result<WalletTxInfoList>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 2, KycType::Undefined);
                mock_user_repo
                    .expect_set_wallet_transactions()
                    .once()
                    .returning(|_, _| Ok(()));
                sdk.repo = Some(Box::new(mock_user_repo));

                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
                    let mut mock_wallet_user = MockWalletUser::new();
                    mock_wallet_user
                        .expect_get_wallet_tx_list()
                        .once()
                        .returning(|_, _| Ok(vec![]));
                    Ok(WalletBorrow::from(mock_wallet_user))
                });
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                    mnemonic_derivation_options: Default::default(),
                });
                sdk.set_networks(example_api_networks());
                sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 0).await;
            }
        }

        // Act
        let response = sdk.get_wallet_tx_list(&PIN, 0, 10).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    fn mock_wallet_transaction(
        hash: String,
        status: WalletTxStatus,
        network_key: String,
        date: DateTime<Utc>,
    ) -> WalletTransaction {
        WalletTransaction {
            date,
            block_number_hash: None,
            transaction_hash: hash,
            receiver: String::new(),
            sender: String::new(),
            amount: unsafe { CryptoAmount::new_unchecked(dec!(20.0)) },
            network_key,
            status,
            explorer_url: None,
            gas_fee: None,
            is_sender: true,
        }
    }

    #[tokio::test]
    async fn test_get_wallet_tx_list_filters_transactions_correctly() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mock_date = Utc::now();

        // During the test, we expect the status of WalletTransaction
        // where transaction_hash = "2" and network_key = "ETH"
        // to transition from 'Pending' to 'Confirmed' after synchronization
        let wallet_transactions_versioned = vec![
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("some tx id"),
                WalletTxStatus::Confirmed,
                String::from("IOTA"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("1"),
                WalletTxStatus::Pending,
                String::from("ETH"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("2"),
                WalletTxStatus::Pending, // this one
                String::from("ETH"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("3"),
                WalletTxStatus::Pending,
                String::from("ETH"),
                mock_date,
            )),
        ];

        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo.expect_get().returning(move |_| {
            Ok(UserEntity {
                user_id: None,
                username: USERNAME.to_string(),
                encrypted_password: Some(ENCRYPTED_WALLET_PASSWORD.clone()),
                salt: SALT.into(),
                is_kyc_verified: false,
                kyc_type: KycType::Undefined,
                viviswap_state: None,
                local_share: None,
                wallet_transactions: Vec::new(),
                wallet_transactions_versioned: wallet_transactions_versioned.clone(),
            })
        });

        let expected = vec![
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("some tx id"),
                WalletTxStatus::Confirmed,
                String::from("IOTA"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("1"),
                WalletTxStatus::Pending,
                String::from("ETH"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("2"),
                WalletTxStatus::Confirmed, // this one
                String::from("ETH"),
                mock_date,
            )),
            VersionedWalletTransaction::from(mock_wallet_transaction(
                String::from("3"),
                WalletTxStatus::Pending,
                String::from("ETH"),
                mock_date,
            )),
        ];

        mock_user_repo
            .expect_set_wallet_transactions()
            .once()
            .with(eq(USERNAME.to_string()), eq(expected.clone()))
            .returning(|_, _| Ok(()));

        sdk.repo = Some(Box::new(mock_user_repo));

        let mut mock_wallet_manager = MockWalletManager::new();
        mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
            let mut mock_wallet_user = MockWalletUser::new();
            mock_wallet_user
                .expect_get_wallet_tx_list()
                .once()
                .returning(|_, _| Ok(vec![]));
            mock_wallet_user
                .expect_get_wallet_tx()
                .once()
                .with(eq(String::from("2"))) // WalletTransaction { transaction_hash = "2" }
                .returning(move |_| {
                    Ok(mock_wallet_transaction(
                        String::from("2"),
                        WalletTxStatus::Confirmed, // Pending -> Confirmed
                        String::from("ETH"),
                        mock_date,
                    ))
                });
            Ok(WalletBorrow::from(mock_wallet_user))
        });

        sdk.active_user = Some(crate::types::users::ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(mock_wallet_manager),
            mnemonic_derivation_options: Default::default(),
        });

        sdk.set_networks(example_api_networks());
        sdk.set_network(ETH_NETWORK_KEY.to_string()).await.unwrap();

        // Act

        // We request a single WalletTransaction using get_wallet_tx_list(start = 1, limit = 1)
        // We have stored transactions: [1 IOTA, 3 ETH]
        // The network key is ETH, so we search through the 3 ETH transactions
        // We select this one:
        // [WalletTransaction{ ... }, -> WalletTransaction{ transaction_hash = "2" }, WalletTransaction{ ... }]
        let response = sdk.get_wallet_tx_list(&PIN, 1, 1).await.unwrap();

        // Assert
        assert_eq!(
            response,
            WalletTxInfoList {
                transactions: vec![mock_wallet_transaction(
                    String::from("2"),
                    WalletTxStatus::Confirmed, // Pending -> Confirmed
                    String::from("ETH"),
                    mock_date,
                )]
            }
        );
    }

    #[tokio::test]
    async fn test_get_wallet_tx_list_does_not_query_network_for_transaction_state() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let wallet_transactions = vec![VersionedWalletTransaction::from(mock_wallet_transaction(
            String::from("1"),
            WalletTxStatus::Confirmed,
            String::from("ETH"),
            Utc::now(),
        ))];

        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo.expect_get().returning(move |_| {
            Ok(UserEntity {
                user_id: None,
                username: USERNAME.to_string(),
                encrypted_password: Some(ENCRYPTED_WALLET_PASSWORD.clone()),
                salt: SALT.into(),
                is_kyc_verified: false,
                kyc_type: KycType::Undefined,
                viviswap_state: None,
                local_share: None,
                wallet_transactions: Vec::new(),
                wallet_transactions_versioned: wallet_transactions.clone(),
            })
        });

        mock_user_repo
            .expect_set_wallet_transactions()
            .once()
            .returning(|_, _| Ok(()));

        sdk.repo = Some(Box::new(mock_user_repo));

        let mut mock_wallet_manager = MockWalletManager::new();
        mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
            let mut mock_wallet_user = MockWalletUser::new();
            mock_wallet_user
                .expect_get_wallet_tx_list()
                .once()
                .returning(|_, _| Ok(vec![]));
            mock_wallet_user.expect_get_wallet_tx().never();
            Ok(WalletBorrow::from(mock_wallet_user))
        });

        sdk.active_user = Some(crate::types::users::ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(mock_wallet_manager),
            mnemonic_derivation_options: Default::default(),
        });

        sdk.set_networks(example_api_networks());
        sdk.set_network(ETH_NETWORK_KEY.to_string()).await.unwrap();

        // Act
        let response = sdk.get_wallet_tx_list(&PIN, 0, 1).await;

        // Assert
        assert!(response.is_ok())
    }

    #[tokio::test]
    async fn test_get_wallet_tx_list_should_sort_wallet_transactions() {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        let tx_3 = VersionedWalletTransaction::from(mock_wallet_transaction(
            String::from("3"),
            WalletTxStatus::Confirmed,
            String::from("ETH"),
            Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 15).unwrap(),
        ));

        let tx_1 = VersionedWalletTransaction::from(mock_wallet_transaction(
            String::from("1"),
            WalletTxStatus::Confirmed,
            String::from("ETH"),
            Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 13).unwrap(),
        ));

        let tx_2 = VersionedWalletTransaction::from(mock_wallet_transaction(
            String::from("2"),
            WalletTxStatus::Confirmed,
            String::from("ETH"),
            Utc.with_ymd_and_hms(2025, 5, 29, 8, 37, 14).unwrap(),
        ));

        let wallet_transactions = vec![tx_3.clone(), tx_1.clone(), tx_2.clone()];
        let expected = vec![
            WalletTransaction::from(tx_3),
            WalletTransaction::from(tx_2),
            WalletTransaction::from(tx_1),
        ];

        let mut mock_user_repo = MockUserRepo::new();
        mock_user_repo.expect_get().returning(move |_| {
            Ok(UserEntity {
                user_id: None,
                username: USERNAME.to_string(),
                encrypted_password: Some(ENCRYPTED_WALLET_PASSWORD.clone()),
                salt: SALT.into(),
                is_kyc_verified: false,
                kyc_type: KycType::Undefined,
                viviswap_state: None,
                local_share: None,
                wallet_transactions: Vec::new(),
                wallet_transactions_versioned: wallet_transactions.clone(),
            })
        });

        mock_user_repo
            .expect_set_wallet_transactions()
            .once()
            .returning(|_, _| Ok(()));

        sdk.repo = Some(Box::new(mock_user_repo));

        let mut mock_wallet_manager = MockWalletManager::new();
        mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
            let mut mock_wallet_user = MockWalletUser::new();
            mock_wallet_user
                .expect_get_wallet_tx_list()
                .once()
                .returning(|_, _| Ok(vec![]));
            mock_wallet_user.expect_get_wallet_tx().never();
            Ok(WalletBorrow::from(mock_wallet_user))
        });

        sdk.active_user = Some(crate::types::users::ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(mock_wallet_manager),
            mnemonic_derivation_options: Default::default(),
        });

        sdk.set_networks(example_api_networks());
        sdk.set_network(ETH_NETWORK_KEY.to_string()).await.unwrap();

        // Act
        let response = sdk.get_wallet_tx_list(&PIN, 0, 5).await;

        // Assert
        assert_eq!(expected, response.unwrap().transactions)
    }
}
