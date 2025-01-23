use crate::{auth::AccessToken, testuser::TestUser};
use serde::{Deserialize, Serialize};

/// Get an access token for the Keycloak admin user
async fn get_admin_access_token() -> Result<AccessToken, Box<dyn std::error::Error>> {
    let username = std::env::var("KC_ADMIN_USERNAME").unwrap();
    let password = std::env::var("KC_ADMIN_PASSWORD").unwrap();
    let keycloak_url = std::env::var("KC_URL").unwrap();
    let realm = std::env::var("KC_REALM").unwrap();

    let url = format!("{keycloak_url}/realms/{realm}/protocol/openid-connect/token");
    let client_id = "admin-cli";

    let params = [
        ("grant_type", "password"),
        ("scope", "profile email openid"),
        ("client_id", client_id),
        ("username", &username),
        ("password", &password),
    ];

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("content-type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await
        .unwrap();

    let access_token = response.json::<AccessToken>().await.unwrap();
    Ok(access_token)
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct KeycloakUser {
    pub username: String,
    pub enabled: bool,
    pub email_verified: bool,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub credentials: Vec<KeycloakCredential>,
    pub groups: Vec<String>,
}

impl From<TestUser> for KeycloakUser {
    fn from(value: TestUser) -> Self {
        Self {
            username: value.username,
            enabled: true,
            email_verified: true,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            credentials: vec![KeycloakCredential {
                temporary: false,
                value: std::env::var("SATOSHI_PASSWORD").unwrap(),
                r#type: "password".to_string(),
            }],
            groups: vec!["tmp".to_string()],
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct KeycloakId {
    pub id: String,
    #[allow(dead_code)] // `username` is never read, but needed for deserialization
    pub username: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct KeycloakCredential {
    pub temporary: bool,
    pub value: String,
    pub r#type: String,
}

impl KeycloakUser {
    pub fn new(username: String, first_name: String, last_name: String, email: String) -> Self {
        Self {
            username,
            enabled: true,
            email_verified: true,
            first_name,
            last_name,
            email,
            credentials: vec![KeycloakCredential {
                temporary: false,
                value: std::env::var("SATOSHI_PASSWORD").unwrap(),
                r#type: "password".to_string(),
            }],
            groups: vec![],
        }
    }

    pub async fn create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let keycloak_url = std::env::var("KC_URL").unwrap();
        let realm = std::env::var("KC_REALM").unwrap();
        let url = format!("{keycloak_url}/admin/realms/{realm}/users");
        let access_token = get_admin_access_token().await?.access_token;

        let client = reqwest::Client::new();
        let response = client
            .post(url)
            .bearer_auth(access_token)
            .header("content-type", "application/json")
            .json(&self)
            .send()
            .await
            .unwrap();

        match response.status() {
            reqwest::StatusCode::CREATED => Ok(()),
            reqwest::StatusCode::CONFLICT => Err("User already exists".into()),
            _ => Err("Failed to create user".into()),
        }
    }

    pub async fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        let keycloak_url = std::env::var("KC_URL").unwrap();
        let realm = std::env::var("KC_REALM").unwrap();
        let access_token = get_admin_access_token().await?.access_token;
        let user_id = get_id_by_username(&self.username, &access_token).await?;

        let url = format!("{keycloak_url}/admin/realms/{realm}/users/{user_id}");
        let client = reqwest::Client::new();
        let response = client.delete(url).bearer_auth(access_token).send().await.unwrap();

        match response.status() {
            reqwest::StatusCode::NO_CONTENT => Ok(()),
            _ => Err("Failed to delete user".into()),
        }
    }
}

/// Get the ID of a user by their username
/// # Arguments
/// * `username` - The username of the user to get the ID for
/// * `access_token` - The access token to use for the request
///
async fn get_id_by_username(username: &str, access_token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let keycloak_url = std::env::var("KC_URL").unwrap();
    let realm = std::env::var("KC_REALM").unwrap();
    let url = format!("{keycloak_url}/admin/realms/{realm}/users?username={username}");

    let client = reqwest::Client::new();
    let response = client.get(url).bearer_auth(access_token).send().await.unwrap();

    let user = response.json::<Vec<KeycloakId>>().await?;
    Ok(user[0].id.clone())
}
