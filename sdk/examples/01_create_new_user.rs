mod utils;
use testing::USER_SATOSHI;
use utils::init_sdk;

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // Temporary
    let m = sdk.print_time().await.unwrap();
    let m_tmp = format!("print_time: {:?}", m);
    println!("{}", m_tmp);

    let c = sdk.debug_config().await.unwrap();
    let c_tmp = format!("debug_config: {:?}", c);
    println!("{}", c_tmp);

    let cc = sdk.check_collision().await.unwrap();
    let cc_tmp = format!("check_collision: {:?}", cc);
    println!("{}", cc_tmp);

    // Create new user
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
}
