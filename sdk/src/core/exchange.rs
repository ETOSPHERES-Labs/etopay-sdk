use super::Sdk;
use crate::{backend::dlt::get_exchange_rate, error::Result};
use log::info;
use rust_decimal::Decimal;

impl Sdk {
    /// Return the current exchange rate.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the exchange rate as a `Decimal` type if successful, or a [`crate::Error`] if an error occurs.
    // MARK10:get_exchange_rate
    pub async fn get_exchange_rate(&self) -> Result<Decimal> {
        info!("Fetching exchange rate from viviswap");
        let _user = self.get_user().await?;

        let access_token = self
            .access_token
            .as_ref()
            .ok_or(crate::error::Error::MissingAccessToken)?;
        let config = self.config.as_ref().ok_or(crate::Error::MissingConfig)?;
        let network = self.active_network.clone().ok_or(crate::Error::MissingNetwork)?;
        let exchange_rate = get_exchange_rate(config, access_token, network.key).await?;
        Ok(exchange_rate)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::core_testing_utils::handle_error_test_cases;
    use crate::testing_utils::{example_api_networks, IOTA_NETWORK_KEY};
    use crate::types::users::ActiveUser;
    use crate::{
        core::Sdk,
        error::Result,
        testing_utils::{
            example_exchange_rate_response, example_get_user, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN,
            USERNAME,
        },
        types::users::KycType,
        wallet_manager::MockWalletManager,
    };
    use api_types::api::viviswap::detail::SwapPaymentDetailKey;
    use mockito::Matcher;
    use rstest::rstest;
    use rust_decimal::Decimal;

    #[rstest]
    #[case::success(Ok(example_exchange_rate_response().course.course.0))]
    #[case::repo_init_error(Err(crate::Error::UserRepoNotInitialized))]
    #[case::unauthorized(Err(crate::Error::MissingAccessToken))]
    #[case::missing_config(Err(crate::Error::MissingConfig))]
    #[tokio::test]
    async fn test_get_exchange_rate(#[case] expected: Result<Decimal>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        let mut mock_server = None;

        match &expected {
            Ok(_) => {
                let mock_user_repo = example_get_user(SwapPaymentDetailKey::Iota, false, 1, KycType::Undefined);
                sdk.repo = Some(Box::new(mock_user_repo));
                sdk.active_user = Some(crate::types::users::ActiveUser {
                    username: USERNAME.into(),
                    wallet_manager: Box::new(MockWalletManager::new()),
                });
                sdk.access_token = Some(TOKEN.clone());
                sdk.set_networks(example_api_networks());
                sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();

                let body = serde_json::to_string(&example_exchange_rate_response()).unwrap();
                mock_server = Some(
                    srv.mock("GET", "/api/courses")
                        .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
                        .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
                        .match_query(Matcher::Exact("network_key=IOTA".to_string()))
                        .with_status(200)
                        .with_body(&body)
                        .expect(1)
                        .create(),
                );
            }
            Err(error) => {
                handle_error_test_cases(error, &mut sdk, 1, 1).await;
            }
        }

        // Act
        let response = sdk.get_exchange_rate().await;

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

    #[tokio::test]
    async fn it_should_get_exchange_rate() {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;
        let mut sdk = Sdk::new(config).unwrap();
        sdk.set_networks(example_api_networks());
        sdk.set_network(IOTA_NETWORK_KEY.to_string()).await.unwrap();

        sdk.repo = Some(Box::new(example_get_user(
            SwapPaymentDetailKey::Iota,
            false,
            1,
            KycType::Undefined,
        )));
        sdk.active_user = Some(get_active_user());
        sdk.access_token = Some(TOKEN.clone());

        // Get exchange rate
        let exchange_rate_mock_response = example_exchange_rate_response();
        let body = serde_json::to_string(&exchange_rate_mock_response).unwrap();
        let get_exchange_rate = srv
            .mock("GET", "/api/courses?network_key=IOTA")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(200)
            .with_body(&body)
            .with_header("content-type", "application/json")
            .with_body(&body)
            .create();

        // Call function you want to test
        let result = sdk.get_exchange_rate().await;

        // Assert
        result.unwrap();
        get_exchange_rate.assert();
    }

    /// Create an active user
    fn get_active_user() -> ActiveUser {
        ActiveUser {
            username: USERNAME.into(),
            wallet_manager: Box::new(MockWalletManager::new()),
        }
    }
}
