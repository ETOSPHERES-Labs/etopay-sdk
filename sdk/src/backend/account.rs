//! Account
//!
//! Everything related to billing and taxation (SAP)
//!

use super::error::{ApiError, Result};
use crate::{core::Config, types::newtypes::AccessToken};
use api_types::api::account::{BusinessPartner, ContractCurrency, Customer, NewCustomer};
use log::{debug, error, info};
use reqwest::StatusCode;

/// Creates a new customer
///
/// # Arguments
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the customer.
/// * `country_code` - The country code of the customer in ISO-3166 Alpha 2 format
///
/// # Errors
///
/// Returns an `Err` variant with the following possible values:
/// * [`ApiError::MissingAccessToken`] if the request is unauthorized.
/// * [`ApiError::UnexpectedResponse`] if an unhandled error occurs.
pub async fn create_new_customer(
    config: &Config,
    access_token: &AccessToken,
    username: &str,
    country_code: &str,
) -> Result<()> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/account/customers");
    info!("Used url: {url:#?}");
    info!("Creating a new customer for {username}");

    let body = NewCustomer {
        country_code: country_code.to_string(),
        business_partner: BusinessPartner::Privat,
        contract_currency: ContractCurrency::EUR,
        vat_id: None,
    };

    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .header("X-APP-USERNAME", username)
        .json(&body)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    match response.status() {
        StatusCode::CREATED => (),
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to create new sap customer: Response status: {}, Response text: {}",
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

/// Get existing customer details
///
/// # Arguments
/// * `config` - The configuration object.
/// * `access_token` - The access token for authentication.
/// * `username` - The username of the customer.
///
/// # Errors
///
/// Returns an `Err` variant with the following possible values:
/// * [`ApiError::MissingAccessToken`] if the request is unauthorized.
/// * [`ApiError::UnexpectedResponse`] if an unhandled error occurs.
pub async fn get_customer_details(config: &Config, access_token: &AccessToken, username: &str) -> Result<Customer> {
    let base_url = &config.backend_url;
    let url = format!("{base_url}/account/customers");
    info!("Used url: {url:#?}");
    info!("Fetching customer for {username}");

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(access_token.as_str())
        .header("X-APP-NAME", &config.auth_provider)
        .header("X-APP-USERNAME", username)
        .send()
        .await?;
    debug!("Response: {response:#?}");

    let customer = match response.status() {
        StatusCode::OK => response.json::<Customer>().await?,
        StatusCode::UNAUTHORIZED => return Err(ApiError::MissingAccessToken),
        _ => {
            let status = response.status();
            let text = response.text().await?;
            error!(
                "Failed to get sap customer details: Response status: {}, Response text: {}",
                status, text
            );
            return Err(ApiError::UnexpectedResponse {
                code: status,
                body: text,
            });
        }
    };

    Ok(customer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing_utils::{
        example_customer, set_config, AUTH_PROVIDER, HEADER_X_APP_NAME, HEADER_X_APP_USERNAME, TOKEN, USERNAME,
    };
    use mockito::Matcher;

    #[rstest::rstest]
    #[case(201, Ok(()))]
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
    async fn test_create_new_customer(#[case] status_code: usize, #[case] expected: Result<()>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_request = NewCustomer {
            country_code: "DE".to_string(),
            business_partner: api_types::api::account::BusinessPartner::Privat,
            contract_currency: api_types::api::account::ContractCurrency::EUR,
            vat_id: None,
        };
        let body = serde_json::to_string(&mock_request).unwrap();

        let mock = srv
            .mock("POST", "/api/account/customers")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .match_header("content-type", "application/json")
            .match_body(Matcher::Exact(body))
            .with_status(status_code)
            .expect(1)
            .with_header("content-type", "application/json")
            .create();

        // Act
        let response = create_new_customer(&config, &TOKEN, USERNAME, "DE").await;

        // Assert
        match expected {
            Ok(_) => response.unwrap(),
            Err(ref err) => {
                assert_eq!(response.unwrap_err().to_string(), err.to_string());
            }
        }
        mock.assert();
    }

    #[rstest::rstest]
    #[case(200, Ok(example_customer()))]
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
    async fn test_get_customer_details(#[case] status_code: usize, #[case] expected: Result<Customer>) {
        // Arrange
        let (mut srv, config, _cleanup) = set_config().await;

        let mock_response = example_customer();
        let body = serde_json::to_string(&mock_response).unwrap();

        let mut mock_server = srv
            .mock("GET", "/api/account/customers")
            .match_header(HEADER_X_APP_NAME, AUTH_PROVIDER)
            .match_header(HEADER_X_APP_USERNAME, USERNAME)
            .match_header("authorization", format!("Bearer {}", TOKEN.as_str()).as_str())
            .with_status(status_code)
            .with_header("content-type", "application/json");
        // Conditionally add the response body only for the 200 status code
        if status_code == 200 {
            mock_server = mock_server.with_body(&body);
        }
        let mock_server = mock_server.expect(1).create();

        // Act
        let response = get_customer_details(&config, &TOKEN, USERNAME).await;

        // Assert
        match expected {
            Ok(expected_customer) => {
                assert_eq!(response.unwrap(), expected_customer);
            }
            Err(ref expected_err) => {
                assert_eq!(response.err().unwrap().to_string(), expected_err.to_string());
            }
        }
        mock_server.assert();
    }
}
