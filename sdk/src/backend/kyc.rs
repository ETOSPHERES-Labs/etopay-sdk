//! Checking kyc status of the user
//!
//! This module provides functionality to check the KYC (Know Your Customer) status of a user.
//! It includes a function `check_kyc_status` that takes the user's access token, username, and configuration as inGET,
//! and returns the KYC status response.
//!

use super::error::{ApiError, Result};
use crate::{core::Config, types::newtypes::AccessToken};
use api_types::api::kyc::KycStatusResponse;
use log::{debug, error, info};
use reqwest::StatusCode;

/// Checking kyc status
///
/// This function checks the KYC status of a user by making a request to the backend API.
/// It requires the user's access token, username, and configuration as inGET.
/// The function returns a `Result` containing the KYC status response or an error.
///
/// # Arguments
///
/// * `config` - The configuration object containing the backend URL and authentication provider name.
/// * `access_token` - The access token of the user.
/// * `username` - The username of the user.
///
/// # Returns
///
/// A `Result` containing the KYC status response (`KycStatusResponse`) or an error ([`ApiError`]).
pub async fn check_kyc_status(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<KycStatusResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/kyc/check-status");
    info!("Used url: {url:#?}");
    info!("Used username: {username}");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    let kyc_status = match response.status() {
        StatusCode::OK => response.json::<KycStatusResponse>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to check kyc status: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(kyc_status)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN, USERNAME};

    fn example_kyc_status_response() -> KycStatusResponse {
        KycStatusResponse {
            username: USERNAME.into(),
            is_verified: true,
        }
    }

    #[rstest::rstest]
    #[case(200, Ok(example_kyc_status_response()))]
    #[case(401, Err(ApiError::MissingAccessToken))]
    #[case(500, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(501, Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_check_kyc_status(#[case] status_code: usize, #[case] expected: Result<KycStatusResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let resp_body = example_kyc_status_response();
        let mock_body_response = serde_json::to_string(&resp_body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/kyc/check-status")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&mock_body_response);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = check_kyc_status(&config, &TOKEN, USERNAME).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(response.unwrap(), resp);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }
}
