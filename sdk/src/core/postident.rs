//! This module includes functions for interacting with postident operations.

use super::Sdk;
use crate::backend::postident::{get_case_details, get_new_case_id, update_case_status};
use crate::error::Result;
use crate::types::users::KycType;
use api_types::api::postident::{CaseDetailsResponse, NewCaseIdResponse};
use log::info;

impl Sdk {
    /// Start kyc verification for postident
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `NewCaseIdResponse` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserRepoNotInitialized`] if the repository fails to initialize.
    /// - [`crate::Error::UserNotInitialized)`] if the user fails to initialize.
    /// - [`crate::Error::UserAlreadyKycVerified`] if the user is already KYC verified.
    pub async fn start_kyc_verification_for_postident(&mut self) -> Result<NewCaseIdResponse> {
        info!("Starting PostIdent Verification for user");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(active_user) = &mut self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let user = repo.get(&active_user.username)?;
        if user.is_kyc_verified {
            return Err(crate::Error::UserAlreadyKycVerified);
        }
        if user.kyc_type == KycType::Undefined {
            repo.set_kyc_type(&active_user.username, KycType::Postident)?;
        } else {
            return Err(crate::Error::UserAlreadyKycVerified);
        }

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let response = get_new_case_id(config, access_token).await?;

        Ok(response)
    }

    /// Get case details for postident
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the `CaseDetailsResponse` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Errors
    ///
    /// - [`crate::Error::UserNotInitialized)`] if the user fails to initialize.
    pub async fn get_kyc_details_for_postident(&self) -> Result<CaseDetailsResponse> {
        info!("Fetching KYC details for postident");
        let Some(user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let case_details = get_case_details(config, access_token).await?;
        Ok(case_details)
    }

    /// Update case status for postident
    ///
    /// # Arguments
    ///
    /// - `case_id`: The ID of the case to update.
    ///
    /// # Errors
    ///
    /// Returns a `Result` containing `()` if successful, or a [`crate::Error`] if an error occurs.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the case status is updated successfully.
    pub async fn update_kyc_status_for_postident(&self, case_id: &str) -> Result<()> {
        info!("updating KYC details for postident");
        let Some(user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;

        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        update_case_status(config, access_token, case_id).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{
        example_case_details, example_get_user, example_new_case_id, set_config, AUTH_PROVIDER, CASE_ID,
        HEADER_X_APP_NAME, TOKEN, USERNAME,
    };
    use crate::{core::Sdk, user::MockUserRepo, wallet_manager::MockWalletManager};
    use api_types::api::postident::{NewCaseIdResponse, UpdateCaseStatusRequest};
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use mockito::Matcher;
    use rstest::rstest;

    #[rstest]
    #[case::success(Ok(example_new_case_id()))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::user_verified_erro(Err(crate::Error::UserAlreadyKycVerified))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_start_kyc_verification_for_postident(#[case] expected: Result<NewCaseIdResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo
                    .expect_set_kyc_type()
                    .times(1)
                    .returning(move |username, typ| {
                        assert_eq!(username, USERNAME);
                        assert_eq!(typ, KycType::Postident);
                        Ok(())
                    });

                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_response = example_new_case_id();
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/postident/get-new-case-id")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_header("content-type", "application/json")
                        .with_body(&body)
                        .expect(1)
                        .create(),
                );
            }
            Err(crate::Error::MissingAccessToken) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo
                    .expect_set_kyc_type()
                    .times(1)
                    .returning(move |username, typ| {
                        assert_eq!(username, USERNAME);
                        assert_eq!(typ, KycType::Postident);
                        Ok(())
                    });
                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = None;
            }
            Err(crate::Error::MissingConfig) => {
                let mut mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                mock_user_repo
                    .expect_set_kyc_type()
                    .times(1)
                    .returning(move |username, typ| {
                        assert_eq!(username, USERNAME);
                        assert_eq!(typ, KycType::Postident);
                        Ok(())
                    });
                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());
                sdk.config = None;
            }
            Err(crate::Error::UserAlreadyKycVerified) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, true, 1, KycType::Postident);
                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 1).await;
            }
        }

        // Act
        let response = sdk.start_kyc_verification_for_postident().await;

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
    #[case::success(Ok(example_case_details()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_postident_kyc_details(#[case] expected: Result<CaseDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_response = example_case_details();
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/postident/get-case-details")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(200)
                        .with_header("content-type", "application/json")
                        .with_body(&body)
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.get_kyc_details_for_postident().await;

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
    #[case::success(Ok(()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_update_postident_kyc_details(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let req = UpdateCaseStatusRequest {
                    case_id: CASE_ID.into(),
                };
                let req = serde_json::to_string(&req).unwrap();

                mock_server = Some(
                    srv.mock("POST", "/api/postident/update-case-status")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .with_status(202)
                        .with_header("content-type", "application/json")
                        .match_body(Matcher::Exact(req))
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.update_kyc_status_for_postident(CASE_ID).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }
}
