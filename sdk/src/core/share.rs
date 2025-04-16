//! The share module provides functionality for dealing with shares of the user wallet mnemonic.

use super::Sdk;
use crate::error::Result;
use crate::share::Share;
use log::info;

impl Sdk {
    /// Get/download the recovery share.
    ///
    /// # Returns
    ///
    /// The recovery share, or `None` if none exists.
    ///
    /// # Error
    ///
    /// Returns error if the user is not initialized.
    pub async fn get_recovery_share(&self) -> Result<Option<Share>> {
        info!("Getting recovery share");
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        Ok(active_user.wallet_manager.get_recovery_share())
    }

    /// Set/upload the recovery share.
    ///
    /// # Arguments
    ///
    /// * `share` - The recovery share to upload.
    ///
    /// # Error
    ///
    /// Returns error if the user is not initialized.
    pub async fn set_recovery_share(&mut self, share: Share) -> Result<()> {
        info!("Setting recovery share");
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        active_user.wallet_manager.set_recovery_share(Some(share));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::{
        core::Sdk,
        error::Result,
        share::Share,
        testing_utils::{USERNAME, set_config},
        wallet_manager::MockWalletManager,
    };
    use mockall::predicate::eq;
    use rstest::rstest;
    use secrecy::ExposeSecret;

    fn example_share() -> String {
        Share::mock_share().to_string().expose_secret().to_owned()
    }

    #[rstest]
    #[case::success(Ok(example_share()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_get_recovery_share(#[case] expected: Result<String>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_get_recovery_share()
                    .once()
                    .returning(|| Some(Share::mock_share()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.get_recovery_share().await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap().unwrap().to_string().expose_secret(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }

    #[rstest]
    #[case::success(Ok(()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[tokio::test]
    async fn test_set_recovery_share(#[case] expected: Result<()>) {
        // Arrange
        let (_srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();

        match &expected {
            Ok(_) => {
                let mut mock_wallet_manager = MockWalletManager::new();
                mock_wallet_manager
                    .expect_set_recovery_share()
                    .once()
                    .with(eq(Some(Share::mock_share())))
                    .returning(|_share| ());
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(mock_wallet_manager),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.set_recovery_share(Share::mock_share()).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
    }
}
