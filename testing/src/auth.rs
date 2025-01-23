use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AccessToken {
    pub access_token: String,
}

/// Fetches an access token for the specified user via keycloak.
/// Required environment variables:
///   - KC_URL
///   - KC_REALM
///   - KC_CLIENT_ID
///   - KC_CLIENT_SECRET
pub async fn get_access_token(username: &str, password: &str) -> AccessToken {
    let keycloak_url = std::env::var("KC_URL").unwrap();
    let realm = std::env::var("KC_REALM").unwrap();
    let url = format!("{keycloak_url}/realms/{realm}/protocol/openid-connect/token");

    let client_id = std::env::var("KC_CLIENT_ID").unwrap();
    let client_secret = std::env::var("KC_CLIENT_SECRET").unwrap();

    let params = [
        ("grant_type", "password"),
        ("scope", "openid"),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("username", username),
        ("password", password),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .unwrap();

    response.json::<AccessToken>().await.unwrap()
}
