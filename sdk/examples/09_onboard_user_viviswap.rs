use etopay_sdk::types::viviswap::{ViviswapVerificationStatus, ViviswapVerificationStep};
use fake::{
    Fake,
    faker::name::{en::LastName, raw::FirstName},
    locales::EN,
};
use std::io::Write;
use testing::USER_SATOSHI;
mod utils;
use utils::init_sdk;

// ---------------------------------------------
// Note: This example will not run until the end because the user already exists in Viviswap db and it will not create a new one.
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

    // Start KYC verification for viviswap
    // The user already exists in viviswap db. Therefore, the test will fail here.
    let new_user = sdk
        .start_kyc_verification_for_viviswap(&format!("{}@gmail.com", user.username), true)
        .await
        .unwrap();
    println!("New Viviswap user: {:#?}", new_user);

    // Get KYC status for viviswap
    let status = sdk.get_kyc_details_for_viviswap().await.unwrap();
    println!("Status: {:#?}", status);

    // Update KYC status for viviswap
    let is_individual = Some(true);
    let is_pep = Some(false);
    let is_us_citizen = Some(false);
    let is_regulatory_disclosure = Some(true);
    let country_of_residence = Some("DE".into());
    let nationality = Some("DE".to_string());
    let full_name = Some(format!(
        "{} {}",
        FirstName(EN).fake::<String>(),
        LastName().fake::<String>()
    ));
    let date_of_birth = Some("2001-11-05".to_string());

    let details = sdk
        .update_kyc_partially_status_for_viviswap(
            is_individual,
            is_pep,
            is_us_citizen,
            is_regulatory_disclosure,
            country_of_residence,
            nationality,
            full_name,
            date_of_birth,
        )
        .await
        .unwrap();
    println!("Details: {:#?}", details);

    sdk.submit_kyc_partially_status_for_viviswap().await.unwrap();

    // Create a waiting loop that prints a dot every 5 seconds for 30 secounds
    println!("Waiting for KYC verification to complete");
    for _ in 0..12 {
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        print!(".");
        std::io::stdout().flush().unwrap();
        let kyc_details = sdk.get_kyc_details_for_viviswap().await.unwrap();
        if kyc_details.verified_step == ViviswapVerificationStep::Personal {
            break;
        }
    }
    println!();

    // Check that the user is verified
    let is_verified = sdk.is_kyc_status_verified(&user.username).await.unwrap();
    println!("IsVerified: {:#?}", is_verified);

    let kyc_details = sdk.get_kyc_details_for_viviswap().await.unwrap();
    println!("KycDetails: {:#?}", kyc_details);
    assert!(kyc_details.verification_status == ViviswapVerificationStatus::Unverified);
    assert!(kyc_details.verified_step == ViviswapVerificationStep::Personal);
    assert!(kyc_details.submission_step == ViviswapVerificationStep::Identity);
}
