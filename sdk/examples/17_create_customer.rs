mod utils;
use testing::USER_SATOSHI;
use utils::init_sdk;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK and create a new user
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create sap customer if not exists
    if sdk.get_customer().await.is_err() {
        sdk.create_customer("DE").await.unwrap();
    } else {
        println!("Customer already exists")
    }
}
