mod utils;
use etopay_sdk::types::newtypes::PlainPassword;
use testing::USER_SATOSHI;
use utils::init_sdk;

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
    sdk.set_wallet_password(
        &user.pin,
        &PlainPassword::try_from_string("correcthorsebatterystaple").unwrap(),
    )
    .await
    .unwrap();
    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Get tx list
    let tx_list = sdk.get_tx_list(0, 10).await.unwrap();
    tx_list
        .txs
        .iter()
        .for_each(|tx| println!("tx reference id: {:?}", tx.reference_id));
}
