mod utils;
use crate::utils::init_sdk;

use sdk::types::viviswap::{ViviswapVerificationStatus, ViviswapVerificationStep};
use std::io::Write;
use testing::USER_ALICE;

#[tokio::test]
#[ignore = "Cause unknown errors, need further investigation"]
async fn should_create_new_almost_verified_user() {
    // Generate a new test user
    let user = testing::TestUser::default();

    // Create the user in keycloak
    let keycloak_user = testing::KeycloakUser::from(user.clone());
    keycloak_user.create().await.unwrap();

    // Initialize SDK and create a new user
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Start KYC verification for the user on viviswap
    let new_user = sdk
        .start_kyc_verification_for_viviswap(&user.email, true)
        .await
        .unwrap();
    println!("New Viviswap user: {:#?}", new_user);

    // Get KYC status of the user
    println!("Waiting for KYC User is available");
    for _ in 0..12 {
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        print!(".");
        std::io::stdout().flush().unwrap();
        if let Ok(kyc_details) = sdk.get_kyc_details_for_viviswap().await {
            println!();
            println!("Kyc_Details: {:#?}", kyc_details);
            break;
        }
    }
    println!();

    // Update KYC status for viviswap
    let is_individual = Some(true);
    let is_pep = Some(false);
    let is_us_citizen = Some(false);
    let is_regulatory_disclosure = Some(true);
    let country_of_residence = Some("DE".into());
    let nationality = Some("DE".to_string());
    let full_name = Some(format!("{} {}", user.first_name, user.last_name));
    let date_of_birth = Some(user.date_of_birth);
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
    // Submit kyc partially status for viviswap
    sdk.submit_kyc_partially_status_for_viviswap().await.unwrap();

    // Create a waiting loop that prints a dot every 5 seconds for 30 seconds
    println!("Waiting for KYC verification to complete");
    for _ in 0..12 {
        tokio::time::sleep(tokio::time::Duration::from_secs(6)).await;
        print!(".");
        std::io::stdout().flush().unwrap();
        if let Ok(kyc_details) = sdk.get_kyc_details_for_viviswap().await {
            if kyc_details.verified_step == ViviswapVerificationStep::Personal {
                break;
            }
        }
    }
    println!();

    let is_verified = sdk.is_kyc_status_verified(&user.username).await.unwrap();
    println!("IsVerified: {:#?}", is_verified);

    let kyc_details = sdk.get_kyc_details_for_viviswap().await.unwrap();
    println!("KycDetails: {:#?}", kyc_details);
    assert!(kyc_details.verification_status == ViviswapVerificationStatus::Unverified);
    assert!(kyc_details.verified_step == ViviswapVerificationStep::Personal);
    assert!(kyc_details.submission_step == ViviswapVerificationStep::Identity);
    // Clean up
    keycloak_user.delete().await.unwrap();
}

#[tokio::test]
#[ignore = "Wait for delete contract implementation"]
async fn should_create_contract() {
    // Initialize SDK and create a new user
    let user: utils::TestUser = (*USER_ALICE).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    // Create wallet
    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Get KYC status of the user
    let status = sdk.get_kyc_details_for_viviswap().await.unwrap();
    println!("Status: {:#?}", status);

    let iban = testing::generate_iban();
    sdk.update_iban_for_viviswap(&user.pin, iban).await.unwrap();

    let iban_details = sdk.get_iban_for_viviswap().await.unwrap();
    println!("IBAN details: {:#?}", iban_details);

    // Get exchange rate
    let exchange_rate = sdk.get_exchange_rate().await.unwrap();
    println!("Exchange rate: {:#?}", exchange_rate);

    // Create deposit
    let deposit = sdk.create_deposit_with_viviswap(&user.pin).await.unwrap();
    println!("Deposit: {:#?}", deposit);

    // TODO: Delete contracts, currently not implemented
}
