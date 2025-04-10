mod utils;
use crate::utils::init_sdk;

use fluent_assertions::*;
use testing::{USER_ALICE, USER_ARCHIVEME, USER_HANS34, USER_HANS48, USER_SATOSHI};

#[tokio::test]
async fn it_should_create_a_new_user() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;
    // Act
    let result = sdk.create_new_user(&user.username).await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_raise_user_already_exists_error() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;
    // Act
    sdk.create_new_user(&user.username).await.unwrap();
    let result = sdk.create_new_user(&user.username).await;
    // Assert
    assert!(result.is_err());
    let err = result.unwrap_err();
    err.should()
        .contain_message("User repository error: User already exists: satoshi");
}

#[tokio::test]
async fn it_should_delete_user_and_wallet() {
    // Arrange
    let user: utils::TestUser = (*USER_ARCHIVEME).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.wallet_password).await.unwrap();

    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    sdk.delete_user(Some(&user.pin)).await.unwrap();

    // Assert
    sdk.create_new_user(&user.username).await.unwrap();
}

#[tokio::test]
async fn user_satoshi_should_be_kyc_verified() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.is_kyc_status_verified(&user.username).await;

    // Assert
    let is_verified = result.unwrap();
    is_verified.should().be_true();
}

#[tokio::test]
async fn user_alice_should_be_kyc_verified() {
    // Arrange
    let user = &USER_ALICE;
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.is_kyc_status_verified(&user.username).await;

    // Assert
    let is_verified = result.unwrap();
    is_verified.should().be_true();
}

#[tokio::test]
async fn user_hans34_should_be_kyc_verified() {
    // Arrange
    let user: utils::TestUser = (*USER_HANS34).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.is_kyc_status_verified(&user.username).await;

    // Assert
    let is_verified = result.unwrap();
    is_verified.should().be_true();
}

#[tokio::test]
async fn user_hans48_should_be_kyc_verified() {
    // Arrange
    let user: utils::TestUser = (*USER_HANS48).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.is_kyc_status_verified(&user.username).await;

    // Assert
    let is_verified = result.unwrap();
    is_verified.should().be_true();
}
