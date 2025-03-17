//! Postident
//!
//! This module provides functions for interacting with the Postident service.
//! It includes functions for getting a new case ID, retrieving case details, and updating case status.

use super::error::{ApiError, Result};
use crate::{core::Config, types::newtypes::AccessToken};
use api_types::api::postident::{CaseDetailsResponse, NewCaseIdResponse, UpdateCaseStatusRequest};
use log::{debug, error, info};
use reqwest::StatusCode;

/// Get a new case ID from the Postident service.
///
/// # Arguments
///
/// * `config` - The configuration object containing the backend URL.
/// * `access_token` - The access token for authentication.
/// * `username` - The username associated with the case.
///
/// # Returns
///
/// Returns a `Result` containing the `NewCaseIdResponse` if successful, or an `Error` if an error occurs.
pub async fn get_new_case_id(config: &Config, access_token: &AccessToken) -> Result<NewCaseIdResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/postident/get-new-case-id");
    info!("Used url: {url:#?}");
    info!("Get new case id");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    let kyc_status = match response.status() {
        StatusCode::OK => response.json::<NewCaseIdResponse>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get new case id: Response status: {}, Response text: {:?}",
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

/// Get case details from the Postident service.
///
/// # Arguments
///
/// * `config` - The configuration object containing the backend URL.
/// * `access_token` - The access token for authentication.
/// * `username` - The username associated with the case.
///
/// # Returns
///
/// Returns a `Result` containing the `CaseDetailsResponse` if successful, or an `Error` if an error occurs.
pub async fn get_case_details(config: &Config, access_token: &AccessToken) -> Result<CaseDetailsResponse> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/postident/get-case-details");
    info!("Used url: {url:#?}");
    info!("Get new case id");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    let case_details = match response.status() {
        StatusCode::OK => response.json::<CaseDetailsResponse>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get case details: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(case_details)
}

/// Update the status of a case in the Postident service.
///
/// # Arguments
///
/// * `config` - The configuration object containing the backend URL.
/// * `username` - The username associated with the case.
/// * `access_token` - The access token for authentication.
/// * `case_id` - The ID of the case to update.
///
/// # Returns
///
/// Returns `Ok(())` if the case status is successfully updated, or an `Error` if an error occurs.
pub async fn update_case_status(config: &Config, access_token: &AccessToken, case_id: &str) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/postident/update-case-status");
    info!("Used url: {url:#?}");
    info!("Update case status for {case_id}");

    let request = UpdateCaseStatusRequest {
        case_id: case_id.into(),
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .json(&request)
        .send()
        .await?;
    debug!("Response: {response:#?}");
    match response.status() {
        StatusCode::ACCEPTED => (),
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to update case status: Response status: {}, Response text: {:?}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        example_case_details, example_new_case_id, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, TOKEN,
    };
    use mockito::Matcher;

    #[rstest::rstest]
    #[case(200, Ok(example_new_case_id()))]
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
    async fn test_new_case_id(#[case] status_code: usize, #[case] expected: Result<NewCaseIdResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = example_new_case_id();
        let mock_body_response = serde_json::to_string(&body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/postident/get-new-case-id")
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
        let response = get_new_case_id(&config, &TOKEN).await;

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

    #[rstest::rstest]
    #[case(200, Ok(example_case_details()))]
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
    async fn test_get_case_details(#[case] status_code: usize, #[case] expected: Result<CaseDetailsResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let body = example_case_details();
        let mock_body_response = serde_json::to_string(&body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/postident/get-case-details")
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
        let response = get_case_details(&config, &TOKEN).await;

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

    #[rstest::rstest]
    #[case(202, Ok(()))]
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
    async fn test_update_case_status(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let case_id = "ABCDEFGH";
        let mock_request = UpdateCaseStatusRequest {
            case_id: case_id.into(),
        };
        let body = serde_json::to_string(&mock_request).unwrap();

        let mock_server = srv
            .mock("POST", "/api/postident/update-case-status")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(body))
            .with_status(status_code)
            .expect(1)
            .create();

        // Act
        let response = update_case_status(&config, &TOKEN, case_id).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        mock_server.assert();
    }
}
