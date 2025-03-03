mod utils;
use sdk::{
    types::{
        currencies::CryptoAmount,
        newtypes::{AccessToken, EncryptionPin, PlainPassword},
        transactions::WalletTxInfoList,
    },
    ErrorKind, WalletError,
};
use testing::USER_SATOSHI;
use utils::{init_sdk, init_sdk_with_cleanup};

#[tokio::test]
async fn it_should_change_password_with_wallet() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.refresh_access_token(None).await.unwrap();
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    assert!(!sdk.is_wallet_password_set().await.unwrap());
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    let new_password = PlainPassword::try_from_string("new_password!").unwrap();
    sdk.set_wallet_password(&user.pin, &new_password).await.unwrap();
}

#[tokio::test]
async fn it_should_change_password_without_wallet() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.refresh_access_token(None).await.unwrap();
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();

    // Act
    assert!(!sdk.is_wallet_password_set().await.unwrap());
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    let new_password = PlainPassword::try_from_string("new_password!").unwrap();
    sdk.set_wallet_password(&user.pin, &new_password).await.unwrap();
}

#[tokio::test]
async fn it_should_create_a_new_wallet_with_access_token() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    // Act
    let result = sdk.create_wallet_from_new_mnemonic(&user.pin).await;

    // Assert
    let mnemonic = result.unwrap();
    assert!(!mnemonic.is_empty());
}

#[tokio::test]
async fn it_should_create_a_new_wallet_without_access_token() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    // the default sdk is initialized with an access token.
    // we call this `refresh_access_token` to undo the access token and put an empty one
    sdk.refresh_access_token(None).await.unwrap();
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    // Act
    let result = sdk.create_wallet_from_new_mnemonic(&user.pin).await;

    // Assert
    let mnemonic = result.unwrap();
    assert!(!mnemonic.is_empty());
}

#[tokio::test]
async fn it_should_verify_mnemonic() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    let mnemonic = sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let result = sdk.verify_mnemonic(&user.pin, &mnemonic).await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_initialize_from_mnemonic() {
    // Arrange

    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Act
    let result = sdk.generate_new_address(&user.pin).await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_generate_new_receiver_address() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let result = sdk.generate_new_address(&user.pin).await;

    // Assert
    let address = result.unwrap();
    assert!(!address.is_empty());
}

#[tokio::test]
async fn it_should_get_balance() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    let _ = sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();
    // Act
    let result = sdk.get_balance(&user.pin).await;
    // Assert

    let balance = result.unwrap();
    assert_eq!(balance, CryptoAmount::ZERO);
}

#[tokio::test]
async fn it_should_fail_get_balance_wrong_pin() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    let _ = sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let wrong_pin = EncryptionPin::try_from_string("54321").unwrap();
    let result = sdk.get_balance(&wrong_pin).await;

    // Assert
    let error = result.unwrap_err();
    assert!(
        matches!(error, sdk::Error::Wallet(WalletError::WrongPinOrPassword)),
        "unexpected error: {error:?}"
    );
}

#[tokio::test]
async fn it_should_create_wallet_backup() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    let _ = sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let backup_password = PlainPassword::try_from_string("backup_password").unwrap();
    let result = sdk.create_wallet_backup(&user.pin, &backup_password).await;

    // Assert
    let backup = result.unwrap();
    assert!(!backup.is_empty());
}

#[tokio::test]
async fn it_should_reset_wallet_pin() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    let _ = sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let new_pin = EncryptionPin::try_from_string("54321").unwrap();
    let result = sdk.change_pin(&user.pin, &new_pin).await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_get_wallet_transaction_list() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // Act
    let result = sdk.get_wallet_tx_list(&user.pin, 0, 10).await;

    // Assert
    let expected_tx_list = WalletTxInfoList { transactions: vec![] };
    assert_eq!(result.unwrap(), expected_tx_list);
}

#[tokio::test]
async fn it_should_fail_getting_balance_without_creating_wallet() {
    dotenvy::dotenv().ok();
    // Arrange

    // Use random user for this test to make sure there is no interference with the other tests
    let user = testing::TestUser::default();
    let keycloak_user = testing::KeycloakUser::from(user.clone());
    keycloak_user.create().await.unwrap();
    let user: utils::TestUser = user.into();

    let (mut sdk, _cleanup) = init_sdk(&user.username).await;
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.delete_wallet(&user.pin).await.unwrap(); // make sure the wallet does not exist

    // Act
    let result = sdk.get_balance(&user.pin).await;
    // Assert

    let error = result.unwrap_err();
    assert!(
        matches!(
            error,
            sdk::Error::Wallet(WalletError::WalletNotInitialized(ErrorKind::UseMnemonic))
        ),
        "unexpected error: {error}"
    );

    // Delete user from KC
    keycloak_user.delete().await.unwrap();
}

#[tokio::test]
async fn it_should_delete_wallet() {
    // Arrange

    let user: utils::TestUser = (*USER_SATOSHI).clone().into();
    let (mut sdk, _cleanup) = init_sdk(&user.username).await;

    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();
    sdk.create_wallet_from_new_mnemonic(&user.pin).await.unwrap();

    // Act
    let result = sdk.delete_wallet(&user.pin).await;

    // Assert
    result.unwrap();
}

#[tokio::test]
async fn it_should_initialize_wallet_from_shares_no_access_token() {
    // Arrange
    let user: utils::TestUser = (*USER_SATOSHI).clone().into();

    // cleanup
    let cleanup = testing::CleanUp::default();

    // config sdk
    let (mut sdk, cleanup) = init_sdk_with_cleanup(&user.username, cleanup).await;
    sdk.refresh_access_token(None).await.unwrap(); // reset the access token

    // create user and wallet
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // extract the recovery share
    let recovery_share = sdk.get_recovery_share().await.unwrap().unwrap();

    // now drop the sdk to simulate restart
    drop(sdk);

    // clean sdk setup but we reuse the cleanup to keep the same folder
    let (mut sdk, _cleanup) = init_sdk_with_cleanup(&user.username, cleanup).await;
    sdk.refresh_access_token(None).await.unwrap(); // reset the access token

    sdk.init_user(&user.username).await.unwrap();

    // initialization from shares should only find the local share
    let out = sdk.get_balance(&user.pin).await;
    assert!(
        matches!(
            out,
            Err(sdk::Error::Wallet(WalletError::WalletNotInitialized(
                ErrorKind::SetRecoveryShare
            )))
        ),
        "{out:?}"
    );

    // put the recovery share back, initialization from shares should only find both shares and pass
    sdk.set_recovery_share(recovery_share).await.unwrap();
    let _ = sdk.generate_new_address(&user.pin).await.unwrap();

    // make sure we have a functioning wallet
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Wallet balance: {balance:?}");
    assert!(balance > CryptoAmount::ZERO); // should always be true for the default mnemonic
}

#[tokio::test]
async fn it_should_initialize_wallet_from_shares_with_access_token() {
    dotenvy::dotenv().ok();

    // Arrange
    let user = testing::TestUser::default();

    // Create the user in keycloak
    let keycloak_user = testing::KeycloakUser::from(user.clone());
    keycloak_user.create().await.unwrap();

    let cleanup = testing::CleanUp::default();

    // config sdk
    let (mut sdk, cleanup) = init_sdk_with_cleanup(&user.username, cleanup).await;

    let user: utils::TestUser = user.into();

    // create user and wallet
    sdk.create_new_user(&user.username).await.unwrap();
    sdk.init_user(&user.username).await.unwrap();
    sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

    sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
        .await
        .unwrap();

    // now drop the sdk to simulate restart
    drop(sdk);

    // clean sdk setup but we reuse the cleanup to keep the same folder
    let (mut sdk, cleanup) = init_sdk_with_cleanup(&user.username, cleanup).await;

    sdk.init_user(&user.username).await.unwrap();

    std::mem::forget(cleanup);

    // initialization from shares should be able to use local and recovery share without password
    let _ = sdk.generate_new_address(&user.pin).await.unwrap();

    // make sure we have a functioning wallet
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Wallet balance: {balance:?}");
    assert!(balance > CryptoAmount::ZERO); // should always be true for the default mnemonic

    // Delete user from KC
    keycloak_user.delete().await.unwrap();
}

#[tokio::test]
async fn it_should_recreate_local_share() {
    dotenvy::dotenv().ok();

    // Arrange
    let user = testing::TestUser::default();

    // Create the user in keycloak
    let keycloak_user = testing::KeycloakUser::from(user.clone());
    keycloak_user.create().await.unwrap();

    // generate access token
    let access_token = testing::get_access_token(&user.username, &user.password)
        .await
        .access_token;
    let access_token = AccessToken::try_from(access_token).unwrap();

    let user: utils::TestUser = user.into();

    {
        let (mut sdk, _cleanup) = init_sdk(&user.username).await;

        sdk.refresh_access_token(Some(access_token.clone())).await.unwrap();

        // create user and wallet
        sdk.create_new_user(&user.username).await.unwrap();
        sdk.init_user(&user.username).await.unwrap();
        sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

        sdk.create_wallet_from_existing_mnemonic(&user.pin, &user.mnemonic)
            .await
            .unwrap();

        // sdk is dropped here to simulate restart, cleanup is also dropped, hence the local share is removed.
    }

    // create fresh sdk instance, use cleanup for next instance
    let (mut sdk, cleanup) = init_sdk(&user.username).await;
    {
        sdk.create_new_user(&user.username).await.unwrap(); // recreate user since we cleaned the db
        sdk.init_user(&user.username).await.unwrap();
        sdk.set_wallet_password(&user.pin, &user.password).await.unwrap();

        // initialization from shares should be able to use recovery and backup share with password
        let _ = sdk.generate_new_address(&user.pin).await.unwrap();
        // make sure we have a functioning wallet
        let balance = sdk.get_balance(&user.pin).await.unwrap();
        println!("Wallet balance: {balance:?}");
        assert!(balance > CryptoAmount::ZERO); // should always be true for the default mnemonic

        // sdk is dropped here to simulate restart, cleanup is still not dropped
        drop(sdk)
    }

    let (mut sdk, cleanup) = init_sdk_with_cleanup(&user.username, cleanup).await;

    sdk.init_user(&user.username).await.unwrap();

    std::mem::forget(cleanup);

    // initialization from shares should be able to use local and recovery share without password
    // (since local share is recreated in the previous wallet initialization)
    let _ = sdk.generate_new_address(&user.pin).await.unwrap();
    // make sure we have a functioning wallet
    let balance = sdk.get_balance(&user.pin).await.unwrap();
    println!("Wallet balance: {balance:?}");
    assert!(balance > CryptoAmount::ZERO); // should always be true for the default mnemonic

    // Delete user from KC
    keycloak_user.delete().await.unwrap();
}
