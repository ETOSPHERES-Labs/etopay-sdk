mod utils;
use crate::utils::init_sdk;

use testing::USER_SATOSHI;

#[tokio::test]
async fn it_should_create_customer_account() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.create_customer("US").await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_get_customer_details() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    let result = sdk.get_customer().await;

    // Assert
    result.unwrap();
}
