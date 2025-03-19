use etopay_sdk::types::newtypes::AccessToken;
mod utils;
use testing::USER_ARCHIVEME;
use utils::init_sdk;

// ---------------------------------------------
// Note: Do not run this example with user `satoshi` because it will then be unverified and it will affect other examples / tests.
// ---------------------------------------------

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_ARCHIVEME).clone().into();

    // the `init_sdk()` function generates an access token for `satoshi`.
    // in this example we use `archiveme` user. Therefore, we generate a new access token for the `archiveme` user.
    let access_token = testing::get_access_token(&user.username, user.password.as_str())
        .await
        .access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();
    sdk.refresh_access_token(Some(access_token)).await.unwrap();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create new wallet
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Delete user
    sdk.delete_user(Some(&user.pin)).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
