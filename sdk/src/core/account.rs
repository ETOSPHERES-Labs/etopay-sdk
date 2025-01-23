//! The account module provides functionality for managing customer accounts.

use super::Sdk;
use crate::backend::account::create_new_customer;
use crate::backend::account::get_customer_details;
use crate::error::Result;
use log::info;

impl Sdk {
    /// Create a new account
    ///
    /// # Arguments
    ///
    /// * `country_code` - The country code for the customer.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the customer account is created successfully.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`crate::Error`] if there is an issue creating the customer account.
    pub async fn create_customer(&self, country_code: &str) -> Result<()> {
        info!("Creating customer account for user");
        let Some(active_user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let username = &active_user.username;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        create_new_customer(config, access_token, username, country_code).await?;

        Ok(())
    }

    /// Get account status
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the customer account details are retrieved successfully.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`crate::Error`] if there is an issue retrieving the customer account details.
    pub async fn get_customer(&mut self) -> Result<()> {
        info!(" Getting account for user");
        let Some(repo) = &mut self.repo else {
            return Err(crate::Error::UserRepoNotInitialized);
        };
        let Some(user) = &self.active_user else {
            return Err(crate::Error::UserNotInitialized);
        };
        let username = &user.username;
        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let response = get_customer_details(config, access_token, username).await?;

        repo.set_customer_details(response)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::error::Result;
    use crate::testing_utils::{
        example_customer, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, HEADER_X_APP_USERNAME, TOKEN, USERNAME,
    };
    use crate::{core::Sdk, user::MockUserRepo, wallet_manager::MockWalletManager};
    use api_types::api::account::NewCustomer;
    use mockall::predicate::eq;
    use mockito::Matcher;
    use rstest::rstest;

    #[rstest]
    #[case::success(Ok(()))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_create_customer(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let country_code = "DE";
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                sdk.repo = Some(Box::new(MockUserRepo::new()));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_request = NewCustomer {
                    country_code: country_code.to_string(),
                    business_partner: api_types::api::account::BusinessPartner::Privat,
                    contract_currency: api_types::api::account::ContractCurrency::EUR,
                    vat_id: None,
                };
                let body = serde_json::to_string(&mock_request).unwrap();

                mock_server = Some(
                    srv.mock("POST", "/api/account/customers")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .match_header("content-type", "application/json")
                        .match_body(Matcher::Exact(body))
                        .with_status(201)
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 0, 0).await;
            }
        }

        // Act
        let response = sdk.create_customer(country_code).await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
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
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::user_init_error(Err(crate::Error::UserNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_customer(#[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mut repo = MockUserRepo::new();
                repo.expect_set_customer_details()
                    .once()
                    .with(eq(example_customer()))
                    .returning(|_| Ok(()));
                sdk.repo = Some(Box::new(repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());

                let mock_response = example_customer();
                let body = serde_json::to_string(&mock_response).unwrap();

                mock_server = Some(
                    srv.mock("GET", "/api/account/customers")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header(HEADER_X_APP_USERNAME, USERNAME)
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
        let response = sdk.get_customer().await;

        // Assert
        match expected {
            Ok(()) => response.unwrap(),
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        if let Some(m) = mock_server {
            m.assert();
        }
    }
}
