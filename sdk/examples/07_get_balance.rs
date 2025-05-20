use etopay_sdk::types::newtypes::PlainPassword;
use testing::USER_SATOSHI;
mod utils;
use utils::init_sdk;

#[allow(clippy::unwrap_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Create / init new wallet from mnemonic
    sdk.set_wallet_password(
        &user.pin,
        &PlainPassword::try_from_string("correcthorsebatterystaple").unwrap(),
    )
    .await
    .unwrap();
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Fetch networks from backend
    let _ = sdk.get_networks().await.unwrap();
    sdk.set_network("iota_rebased_testnet".to_string()).await.unwrap();

    // Generate address
    let address = sdk.generate_new_address(&user.pin).await.unwrap();
    println!("Address: {}", address);

    // Get balance
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Balance: {:?}", balance);
}
