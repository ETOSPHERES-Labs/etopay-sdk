use super::error::{ApiError, Result};
use crate::types::newtypes::AccessToken;
use crate::{core::Config, share::Share};
use api_types::api::user::GetShareResponse;
use api_types::api::user::PutSharesRequest;
use log::{debug, error, info};
use reqwest::StatusCode;
use secrecy::ExposeSecret;

/// Uploads the backup and recovery shares
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `backup_share` - The backup share.
/// * `recovery_share` - The recovery share.
///
/// # Returns
///
/// Returns an empty `Result` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `ApiError::MissingAccessToken` if the request is unauthorized, `ApiError::ShareError` if the share is not encrypted
/// or an `ApiError::UnexpectedResponse` if an unexpected error occurs.
pub async fn upload_shares(
    config: &Config,
    access_token: &AccessToken,
    backup_share: &Share,
    recovery_share: &Share,
) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/shares");
    info!("Used url: {url:#?}");

    // Double check if the share is encrypted
    if !backup_share.is_encrypted() {
        return Err(ApiError::Share("Backup share is not encrypted".to_string()));
    }

    let body = PutSharesRequest {
        backup_share: backup_share.to_string().expose_secret().to_owned(),
        recovery_share: recovery_share.to_string().expose_secret().to_owned(),
    };

    info!("Uploading backup shares");

    let client = reqwest::Client::new();
    let response = client
        .put(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .json(&body)
        .send()
        .await?;
    debug!("Upload backup share response: {response:#?}");

    match response.status() {
        StatusCode::OK => Ok(()),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to upload the backup share: Response status: {}, Response text: {:?}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Download the backup share
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The corresponding user for the share.
///
/// # Returns
///
/// Returns a `Result` containing the backup share if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// * `ApiError::MissingAccessToken` if the request is unauthorized.
/// * `ApiError::ShareError` if an unhandled error occurs.
/// * `ApiError::ParseError` if it's not possible to parse the string share.
pub async fn download_backup_share(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<Option<Share>> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/shares/backup");
    info!("Used url: {url:#?}");

    info!("Downloading backup share for user {}", username);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Download backup share response: {response:#?}");

    match response.status() {
        StatusCode::OK => {
            let str_share = response.json::<GetShareResponse>().await?.share;
            let backup_share = str_share.parse::<Share>().map_err(|e| ApiError::Parse(e.to_string()))?;
            Ok(Some(backup_share))
        }
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to download the backup share: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Download the recovery share
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The corresponding user for the share.
///
/// # Returns
///
/// Returns a `Result` containing the recovery share if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// * `ApiError::MissingAccessToken` if the request is unauthorized.
/// * `ApiError::ShareError` if an unhandled error occurs.
/// * `ApiError::ParseError` if it's not possible to parse the string share.
pub async fn download_recovery_share(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
) -> Result<Option<Share>> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/shares/recovery");
    info!("Used url: {url:#?}");

    info!("Downloading recovery share for user {}", username);

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Download recovery share response: {response:#?}");

    match response.status() {
        StatusCode::OK => {
            let str_share = response.json::<GetShareResponse>().await?.share;
            let recovery_share = str_share.parse::<Share>().map_err(|e| ApiError::Parse(e.to_string()))?;
            Ok(Some(recovery_share))
        }
        StatusCode::NOT_FOUND => Ok(None),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to download the recovery share: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

/// Delete user shares
///
/// # Arguments
///
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The corresponding user for the share.
///
/// # Returns
///
/// Returns an empty `Result` if successful, or an `Error` if an error occurs.
///
/// # Errors
///
/// Returns an `ApiError::MissingAccessToken` if the request is unauthorized, or an `ApiError::UnexpectedResponse` if an unhandled error occurs.
pub async fn delete_shares(config: &Config, access_token: &AccessToken, username: &str) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/user/shares");
    info!("Used url: {url:#?}");

    info!("Deleting shares for user {}", username);

    let client = reqwest::Client::new();
    let response = client
        .delete(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .send()
        .await?;
    debug!("Delete shares response: {response:#?}");

    match response.status() {
        StatusCode::OK => Ok(()),
        StatusCode::UNAUTHORIZED => Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to delete user shares: Response status: {}, Response text: {}",
                status, text
            );
            Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        set_config, AUTH_PROVIDER, ENCRYPTED_SHARE, HEADER_X_APP_NAME, NOT_ENCRYPTED_SHARE, TOKEN, USERNAME,
    };
    use mockito::Matcher;
    use secrecy::ExposeSecret;

    fn example_share_response() -> GetShareResponse {
        GetShareResponse {
            share: ENCRYPTED_SHARE.into(),
        }
    }

    #[rstest::rstest]
    #[case(200, ENCRYPTED_SHARE, NOT_ENCRYPTED_SHARE, Ok(()))]
    #[case(401, ENCRYPTED_SHARE, NOT_ENCRYPTED_SHARE, Err(ApiError::MissingAccessToken))]
    #[case(500, ENCRYPTED_SHARE, NOT_ENCRYPTED_SHARE, Err(ApiError::UnexpectedResponse {
        code: StatusCode::INTERNAL_SERVER_ERROR,
        body: "".to_string() 
    }))]
    #[case(500, NOT_ENCRYPTED_SHARE,NOT_ENCRYPTED_SHARE,  Err(ApiError::Share("Backup share is not encrypted".to_string())))]
    #[case(501, ENCRYPTED_SHARE,NOT_ENCRYPTED_SHARE,  Err(ApiError::UnexpectedResponse {
        code: StatusCode::NOT_IMPLEMENTED,
        body: "".to_string() 
    }))]
    #[tokio::test]
    async fn test_upload_shares(
        #[case] status_code: usize,
        #[case] str_share: &str,
        #[case] str_share2: &str,
        #[case] expected: Result<()>,
    ) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = PutSharesRequest {
            backup_share: ENCRYPTED_SHARE.into(),
            recovery_share: NOT_ENCRYPTED_SHARE.into(),
        };
        let body = serde_json::to_string(&mock_request).unwrap();

        let mut mock_server = srv
            .mock("PUT", "/api/user/shares")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_body(Matcher::Exact(body))
            .with_status(status_code);
        // Skip call if share is not encrypted
        if str_share == NOT_ENCRYPTED_SHARE {
            mock_server = mock_server.expect(0);
        } else {
            mock_server = mock_server.expect(1);
        }
        let mock_server = mock_server.create();

        // Act
        let backup_share = str_share.parse::<Share>().unwrap();
        let recovery_share = str_share2.parse::<Share>().unwrap();
        let response = upload_shares(&config, &TOKEN, &backup_share, &recovery_share).await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(example_share_response()))]
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
    async fn test_download_backup_share(#[case] status_code: usize, #[case] expected: Result<GetShareResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let resp_body = example_share_response();
        let mock_body_response = serde_json::to_string(&resp_body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/user/shares/backup")
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
        let response = download_backup_share(&config, &TOKEN, USERNAME).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(
                    response.unwrap().unwrap().to_string().expose_secret().to_owned(),
                    resp.share
                );
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(example_share_response()))]
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
    async fn test_download_recovery_share(#[case] status_code: usize, #[case] expected: Result<GetShareResponse>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let resp_body = example_share_response();
        let mock_body_response = serde_json::to_string(&resp_body).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/user/shares/recovery")
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
        let response = download_recovery_share(&config, &TOKEN, USERNAME).await;

        // Assert
        match expected {
            Ok(resp) => {
                assert_eq!(
                    response.unwrap().unwrap().to_string().expose_secret().to_owned(),
                    resp.share
                );
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(()))]
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
    async fn test_delete_shares(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        // mock call to endpoint
        let mock_server = srv
            .mock("DELETE", "/api/user/shares")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .expect(1)
            .with_status(status_code)
            .create();

        // Act
        let response = delete_shares(&config, &TOKEN, USERNAME).await;

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
