#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(dead_code)]

use etopay_sdk::{
    core::{Config, Sdk},
    types::newtypes::{AccessToken, EncryptionPin, PlainPassword},
};
use std::path::Path;
use testing::CleanUp;

/// initialize sdk from an existing [`CleanUp`] object as the storage path.
pub async fn init_sdk_with_cleanup(username: &str, existing_cleanup: CleanUp) -> (Sdk, CleanUp) {
    dotenvy::dotenv().ok();

    let password = std::env::var("SATOSHI_PASSWORD").unwrap();

    let backend_url =
        std::env::var("RT_API_URL").expect("RT_API_URL should be set with the backend url for the tests to use");

    // construct the config to use for the SDK
    let config = Config {
        backend_url: backend_url.parse().expect("RT_API_URL must be a valid URL"),
        path_prefix: Path::new(&existing_cleanup.path_prefix).into(),
        auth_provider: "standalone".to_string(),
        log_level: log::LevelFilter::Debug,
    };

    let mut sdk = Sdk::new(config).expect("should not fail to initialize sdk"); // set the backend url if the environment variable is set

    // generate access token
    let access_token = testing::get_access_token(username, &password).await.access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    // Fetch networks from backend
    let _ = sdk.get_networks().await.unwrap();
    sdk.set_network("iota_rebased_testnet".to_string()).await.unwrap();

    (sdk, existing_cleanup)
}

/// initialize sdk with a new [`CleanUp`] object as the storage path.
pub async fn init_sdk(username: &str) -> (Sdk, CleanUp) {
    init_sdk_with_cleanup(username, CleanUp::default()).await
}

/// A copy of the [`testing::TestUser`] that uses the newtype wrappers for sensitive values for
/// easier use in the examples.
#[derive(Debug)]
pub struct TestUser {
    pub username: String,
    pub wallet_password: PlainPassword,
    pub pin: EncryptionPin,
    pub mnemonic: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub date_of_birth: String,
    pub iban: String,
}

impl From<testing::TestUser> for TestUser {
    fn from(value: testing::TestUser) -> Self {
        Self {
            username: value.username,
            wallet_password: PlainPassword::try_from_string("correcthorsebatterystaple").unwrap(),
            pin: EncryptionPin::try_from_string(value.pin).unwrap(),
            mnemonic: value.mnemonic,
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            date_of_birth: value.date_of_birth,
            iban: value.iban,
        }
    }
}
