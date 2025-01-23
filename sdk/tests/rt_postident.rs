mod utils;
use crate::utils::init_sdk;

#[tokio::test]
async fn it_should_onboard_with_postident_and_get_case_details_and_update_status() {
    // Arrange
    let user = testing::TestUser::default();

    // Create the user in keycloak
    let keycloak_user = testing::KeycloakUser::from(user.clone());
    keycloak_user.create().await.unwrap();

    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Exit if user is already verified
    let is_verified = sdk.is_kyc_status_verified(&user.username).await.unwrap();
    if is_verified {
        println!("User is already verified. No need to delete. Exiting");
        return;
    }

    // Act onboard
    let result = sdk.start_kyc_verification_for_postident().await;
    let new_case = result.unwrap();
    assert!(!new_case.case_id.is_empty());

    // Act case details
    let result = sdk.get_kyc_details_for_postident().await;
    let case_details = result.unwrap();
    assert_eq!(new_case.case_id, case_details.case_id);

    // Act update status
    let result = sdk.update_kyc_status_for_postident(&new_case.case_id).await;
    result.unwrap();

    // Delete user from KC
    keycloak_user.delete().await.unwrap();
}
