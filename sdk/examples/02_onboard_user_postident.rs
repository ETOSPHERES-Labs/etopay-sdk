mod utils;
use testing::USER_SATOSHI;
use utils::init_sdk;

// ---------------------------------------------
// Note: This examples does not work unless you do manual postident verification at https://postident-itu.deutschepost.de/testapp
// ---------------------------------------------

#[allow(clippy::unwrap_used, clippy::expect_used)]
#[tokio::main]
async fn main() {
    // Initialize SDK and create a new user
    let (mut sdk, _cleanup) = init_sdk().await;
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Exit if user is already verified
    let is_verified = sdk.is_kyc_status_verified(&user.username).await.unwrap();
    if is_verified {
        println!("User is already verified, please run the delete_user example first.");
        return;
    }

    // Start KYC verification for postident
    let new_case_id = sdk.start_kyc_verification_for_postident().await.unwrap();
    println!("New postident user with case: {:#?}", new_case_id);

    // Do manual postident verification at
    // https://postident-itu.deutschepost.de/testapp
    let mut enter = String::new();
    println!("Do postident KYC and hit enter to continue...");
    std::io::stdin()
        .read_line(&mut enter)
        .expect("error: unable to read user input");

    // Finish KYC verification for postident
    sdk.update_kyc_status_for_postident(&new_case_id.case_id).await.unwrap();

    // Check that the user is verified
    let is_verified = sdk.is_kyc_status_verified(&user.username).await.unwrap();
    println!("IsVerified: {:#?}", is_verified);
}
