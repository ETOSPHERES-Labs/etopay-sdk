use sdk::types::newtypes::EncryptionPin;
mod utils;
use testing::USER_SATOSHI;
use utils::init_sdk;

// ---------------------------------------------
// Note: This examples does not work. It gets stuck. Does not pass or fail.
// ---------------------------------------------

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create new wallet
    sdk.set_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Reset pin
    let new_pin = EncryptionPin::try_from_string("123456").unwrap();
    sdk.change_pin(&user.pin, &new_pin).await.unwrap();

    // Verify pin
    sdk.verify_pin(&new_pin).await.unwrap();
}
