mod utils;
use api_types::api::networks::{ApiNetwork, ApiProtocol};
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
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Set network
    sdk.set_networks(vec![ApiNetwork {
        key: "iota-rebased".to_string(),
        is_testnet: true,
        display_name: "IOTA Rebased".to_string(),
        display_symbol: "IOTAR".to_string(),
        coin_type: 0,
        // node_urls: vec!["http://127.0.0.1:9000".to_string()],
        // node_urls: vec!["https://api.devnet.iota.cafe".to_string()],
        node_urls: vec!["https://api.testnet.iota.cafe".to_string()],
        decimals: 9,
        can_do_purchases: false,
        protocol: ApiProtocol::IotaRebased {
            coin_type: "0x2::iota::IOTA".to_string(),
        },
        block_explorer_url: String::new(),
    }]);
    sdk.set_network("iota-rebased".to_string()).await.unwrap();

    // Generate new address
    let recipient_address = sdk.generate_new_address(&user.pin).await.unwrap();
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("address: {recipient_address}, balance: {balance:?}");

    let recipient_address = "0x393bdb466e8efc29ca5905f23d90b069d2b00f35016dd316039c97b0363ce217".to_string();

    // Send amount
    let amount = dec!(1.0).try_into().unwrap();
    let data = Some("test".to_string().into_bytes());
    // estimate gas
    // let estimate = sdk
    //     .estimate_gas(&user.pin, &recipient_address, amount, data.clone())
    //     .await
    //     .unwrap();
    //
    // println!("Estimated gas: {estimate:?}");

    let tx_id = sdk
        .send_amount(&user.pin, &recipient_address, amount, data)
        .await
        .unwrap();

    println!("Success with transaction id: {tx_id}");

    let details = sdk.get_wallet_tx(&user.pin, &tx_id).await.unwrap();
    println!("Details:\n{:#?}", details);
}
