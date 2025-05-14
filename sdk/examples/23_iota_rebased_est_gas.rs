mod utils;

use etopay_sdk::types::newtypes::PlainPassword;
use rust_decimal_macros::dec;
use testing::USER_SATOSHI;
use utils::init_sdk;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk().await;

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

    let custom_mnemonic = "";
    // sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &custom_mnemonic)
        .await
        .unwrap();

    // Fetch networks from backend
    let networks = sdk.get_networks().await.unwrap();

    println!("networks: {:?}", networks.get(2));
    let iota_network_key = &networks.get(2).unwrap().key;
    sdk.set_network(iota_network_key.to_string()).await.unwrap();

    // // Generate new address
    let recipient_address = sdk.generate_new_address(&user.pin).await.unwrap();
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("address: {recipient_address}, balance: {balance:?}");

    let amount = dec!(0.1).try_into().unwrap();
    let est = sdk
        .estimate_gas(&user.pin, &recipient_address, amount, None)
        .await
        .unwrap();

    println!("est: {:?}", est);
}
