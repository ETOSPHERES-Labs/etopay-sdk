//! Main implementation for the [`UserRepo`], which uses a simple storage abstraction internally.

use super::UserRepo;
use super::error::Result;
use crate::{
    share::Share,
    types::{
        newtypes::EncryptedPassword,
        transactions::WalletTxInfo,
        users::{KycType, UserEntity},
        viviswap::{ViviswapPartiallyKycDetails, ViviswapState, ViviswapVerificationStatus, ViviswapVerificationStep},
    },
    user::error::UserKvStorageError,
};
use log::debug;

pub struct UserRepoImpl<I: super::UserKvStorage> {
    inner: I,
}

impl<I: super::UserKvStorage> UserRepoImpl<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I: super::UserKvStorage> UserRepo for UserRepoImpl<I> {
    fn create(&mut self, user: &UserEntity) -> Result<()> {
        debug!("Creating entry in user DB");
        if self.inner.exists(&user.username)? {
            return Err(UserKvStorageError::UserAlreadyExists {
                username: user.username.clone(),
            })?;
        }

        self.inner.set(&user.username, user)?;
        Ok(())
    }

    fn update(&mut self, user: &UserEntity) -> Result<()> {
        debug!("Updating entry in user DB");

        if !self.inner.exists(&user.username)? {
            return Err(UserKvStorageError::UserNotFound {
                username: user.username.clone(),
            })?;
        }
        self.inner.set(&user.username, user)
    }

    fn delete(&mut self, username: &str) -> Result<()> {
        debug!("Deleting entry in user DB");
        self.inner.delete(username)
    }

    fn get(&self, username: &str) -> Result<UserEntity> {
        debug!("Fetching entry in user DB");
        self.inner.get(username)
    }

    fn set_wallet_password(&mut self, username: &str, password: EncryptedPassword) -> Result<()> {
        debug!("Setting password in user DB");

        let mut user = self.inner.get(username)?;
        user.encrypted_password = Some(password.to_owned());
        self.inner.set(username, &user)
    }

    fn set_kyc_state(&mut self, username: &str, is_verified: bool) -> Result<()> {
        debug!("Setting KYC state in user DB: {is_verified}");

        let mut user = self.inner.get(username)?;
        user.is_kyc_verified = is_verified;
        self.inner.set(username, &user)
    }

    fn set_kyc_type(&mut self, username: &str, kyc_type: KycType) -> Result<()> {
        debug!("Setting KYC type in user DB: {kyc_type:#?}");

        let mut user = self.inner.get(username)?;
        user.kyc_type = kyc_type;
        self.inner.set(username, &user)
    }

    fn set_viviswap_kyc_state(
        &mut self,
        username: &str,
        verification_status: ViviswapVerificationStatus,
        monthly_limit_eur: f32,
        next_verification_step: ViviswapVerificationStep,
    ) -> Result<()> {
        debug!(
            "Setting viviswap KYC state in user DB: {verification_status:?}, {monthly_limit_eur}, {next_verification_step:?}"
        );

        let mut user = self.inner.get(username)?;

        match user.viviswap_state {
            None => {
                user.viviswap_state = Some(ViviswapState {
                    verification_status,
                    monthly_limit_eur,
                    next_verification_step,
                    partial_kyc_details_input: ViviswapPartiallyKycDetails::new(),
                    current_iban: Option::None,
                    payment_methods: Option::None,
                });
            }
            Some(viviswap_state) => {
                user.viviswap_state = Some(ViviswapState {
                    verification_status,
                    monthly_limit_eur,
                    next_verification_step,
                    partial_kyc_details_input: viviswap_state.partial_kyc_details_input,
                    current_iban: viviswap_state.current_iban,
                    payment_methods: viviswap_state.payment_methods,
                });
            }
        };
        self.inner.set(username, &user)
    }

    fn set_local_share(&mut self, username: &str, share: Option<&Share>) -> Result<()> {
        use secrecy::ExposeSecret;
        debug!("Setting local share in user DB for: {username}");
        let mut user = self.inner.get(username)?;
        user.local_share = share.map(|s| s.to_string().expose_secret().to_string());
        self.inner.set(username, &user)
    }

    fn set_wallet_transactions(&mut self, username: &str, transaction: Vec<WalletTxInfo>) -> Result<()> {
        debug!("Setting wallet transactions in user DB: {transaction:#?}");
        let mut user = self.inner.get(username)?;
        user.wallet_transactions = transaction;
        self.inner.set(username, &user)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use iota_sdk::wallet::account::types::InclusionState;

    use super::*;
    use crate::{
        testing_utils::{ENCRYPTED_WALLET_PASSWORD, ETH_NETWORK_KEY},
        types::{
            newtypes::{EncryptedPassword, EncryptionPin, EncryptionSalt, PlainPassword},
            users::KycType,
        },
        user::memory_storage::MemoryUserStorage,
    };

    fn create_user_entity(username: &str, password: Option<EncryptedPassword>) -> UserEntity {
        UserEntity {
            user_id: None,
            username: username.to_owned(),
            encrypted_password: password,
            salt: EncryptionSalt::generate(),
            is_kyc_verified: false,
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        }
    }

    #[test]
    fn it_should_create_a_new_user_db() {
        // Arrange
        let username = "hauju";

        let user = create_user_entity(username, None);

        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        // Act
        let result = user_repo.create(&user);
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn it_should_return_the_user() {
        // Arrange
        let username = "hauju";

        let user = create_user_entity(username, Some(ENCRYPTED_WALLET_PASSWORD.clone()));

        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();
        // Act
        let ret_user = user_repo.get(username).unwrap();
        // Assert
        assert_eq!(ret_user, user);
    }

    #[test]
    fn it_should_set_wallet_password() {
        // Arrange
        let username = "hauju";
        let pin = EncryptionPin::try_from_string("123456").unwrap();
        let plain_password = "correcthorsebatterystaple";

        let user = create_user_entity(username, None);

        let encrypted_password = PlainPassword::try_from_string(plain_password)
            .unwrap()
            .encrypt(&pin, &user.salt)
            .unwrap();

        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());

        user_repo.create(&user).unwrap();
        // Act
        let result = user_repo.set_wallet_password(username, encrypted_password.clone());
        // Assert
        assert!(result.is_ok());
        let user = user_repo.get(username).unwrap();
        assert!(user.encrypted_password.is_some());
        assert!(user.encrypted_password == Some(encrypted_password));
    }

    #[test]
    fn it_should_raise_error_if_user_already_exists() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);

        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();
        // Act
        let result = user_repo.create(&user);
        // Assert
        assert!(matches!(
            result.unwrap_err(),
            UserKvStorageError::UserAlreadyExists { .. }
        ));
    }

    #[test]
    fn it_should_set_new_viviswap_state() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();
        let viviswap_state = ViviswapState {
            verification_status: ViviswapVerificationStatus::PartiallyVerified,
            monthly_limit_eur: 500.0,
            next_verification_step: ViviswapVerificationStep::Residence,
            partial_kyc_details_input: ViviswapPartiallyKycDetails::default(),
            current_iban: Option::None,
            payment_methods: Option::None,
        };
        let expected_state = viviswap_state.clone();
        // Act
        let result = user_repo.set_viviswap_kyc_state(
            &username,
            viviswap_state.verification_status,
            viviswap_state.monthly_limit_eur,
            viviswap_state.next_verification_step,
        );
        // Assert
        let user = user_repo.get(&username).unwrap();
        assert!(result.is_ok());
        assert!(user.viviswap_state == Some(expected_state));
    }

    #[test]
    fn it_should_update_existing_user_postident() {
        // Arrange
        let username = String::from("hauju");
        let new_salt = EncryptionSalt::generate();

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let updated_user = UserEntity {
            user_id: None,
            username: username.clone(),
            encrypted_password: None,
            salt: new_salt.clone(),       // New salt value for update
            is_kyc_verified: true,        // New KYC verification status
            kyc_type: KycType::Undefined, // New KYC type
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        };
        let result = user_repo.update(&updated_user);

        // Assert
        assert!(result.is_ok());
        let retrieved_user = user_repo.get(&username).unwrap();
        assert_eq!(retrieved_user.salt, new_salt);
        assert_eq!(retrieved_user.kyc_type, KycType::Undefined);
    }
    #[test]
    fn it_should_update_existing_user_viviswap() {
        // Arrange
        let username = String::from("hauju");

        let new_salt = EncryptionSalt::generate();

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let updated_user = UserEntity {
            user_id: None,
            username: username.clone(),
            encrypted_password: None,
            salt: new_salt.clone(),       // New salt value for update
            is_kyc_verified: true,        // New KYC verification status
            kyc_type: KycType::Undefined, // New KYC type
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        };
        let result = user_repo.update(&updated_user);

        // Assert
        assert!(result.is_ok());
        let retrieved_user = user_repo.get(&username).unwrap();
        assert_eq!(retrieved_user.salt, new_salt);
        assert_eq!(retrieved_user.kyc_type, KycType::Undefined);
    }
    #[test]
    fn it_should_delete_existing_user() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let result = user_repo.delete(&username);

        // Assert
        assert!(result.is_ok());
        assert!(user_repo.get(&username).is_err());
    }
    #[test]
    fn it_should_set_user_kyc_type() {
        // Arrange
        let username = String::from("hauju");
        let user = UserEntity {
            user_id: None,
            username: username.clone(),
            encrypted_password: None,
            salt: EncryptionSalt::generate(),
            is_kyc_verified: false,
            // Add the actual field for storing KYC type
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        };
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let new_kyc_type = KycType::Undefined;
        let result = user_repo.set_kyc_type(&username, new_kyc_type.clone());

        // Assert
        assert!(result.is_ok());
        let retrieved_user = user_repo.get(&username).unwrap();
        // Adjust the assertion based on the actual field in UserEntity
        assert_eq!(retrieved_user.kyc_type, new_kyc_type);
    }
    #[test]
    fn it_should_set_user_kyc_state() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let new_kyc_state = true;
        let result = user_repo.set_kyc_state(&username, new_kyc_state);

        // Assert
        assert!(result.is_ok());
        let retrieved_user = user_repo.get(&username).unwrap();
        assert_eq!(retrieved_user.is_kyc_verified, new_kyc_state);
    }

    #[test]
    fn it_should_set_local_share() {
        use secrecy::ExposeSecret;

        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        // Act
        let share = "ME-RS-N-Mi0yLUNBSVFBaGdESXFBRWk3b296TFVtbzNscG1jZEIx".parse().unwrap();
        let result = user_repo.set_local_share(&username, Some(&share));

        // Assert
        result.unwrap();
        let retrieved_user = user_repo.get(&username).unwrap();
        assert_eq!(&retrieved_user.local_share.unwrap(), share.to_string().expose_secret());
    }

    #[test]
    fn it_should_update_wallet_transactions() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());
        user_repo.create(&user).unwrap();

        let txs = vec![
            WalletTxInfo {
                date: String::new(),
                block_id: None,
                transaction_id: String::from("transaction_id_1"),
                receiver: String::new(),
                incoming: false,
                amount: 0.5,
                network_key: ETH_NETWORK_KEY.to_string(),
                status: format!("{:?}", InclusionState::Pending),
                explorer_url: None,
            },
            WalletTxInfo {
                date: String::new(),
                block_id: Some(String::from("block_2")),
                transaction_id: String::from("transaction_id_2"),
                receiver: String::new(),
                incoming: true,
                amount: 4.0,
                network_key: ETH_NETWORK_KEY.to_string(),
                status: format!("{:?}", InclusionState::Pending),
                explorer_url: None,
            },
        ];

        // Act
        let _ = user_repo.set_wallet_transactions(&username, txs.clone());

        let retrieved_user = user_repo.get(&username).unwrap();

        // Assert
        assert_eq!(retrieved_user.wallet_transactions.len(), 2);

        assert_eq!(
            retrieved_user.wallet_transactions.first().unwrap(),
            txs.first().unwrap()
        );

        assert_eq!(retrieved_user.wallet_transactions.get(1).unwrap(), txs.get(1).unwrap());
    }

    #[test]
    fn it_should_error_on_duplicate_user_creation() {
        // Arrange
        let username = String::from("hauju");

        let user = create_user_entity(&username, None);
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());

        // Act
        let create_result_1 = user_repo.create(&user);
        let create_result_2 = user_repo.create(&user);

        // Assert
        assert!(create_result_1.is_ok());
        assert!(matches!(
            create_result_2.unwrap_err(),
            UserKvStorageError::UserAlreadyExists { .. }
        ));
    }
    #[test]
    fn it_should_error_on_get_nonexistent_user() {
        // Arrange
        let username = String::from("nonexistent_user");
        let user_repo = UserRepoImpl::new(MemoryUserStorage::new());

        // Act
        let get_result = user_repo.get(&username);

        // Assert
        assert!(matches!(
            get_result.unwrap_err(),
            UserKvStorageError::UserNotFound { .. }
        ));
    }
    #[test]
    fn it_should_error_on_update_nonexistent_user() {
        // Arrange
        let username = String::from("nonexistent_user");
        let user = UserEntity {
            user_id: None,
            username: username.clone(),
            encrypted_password: None,
            salt: EncryptionSalt::generate(),
            is_kyc_verified: true,
            kyc_type: KycType::Undefined,
            viviswap_state: None,
            local_share: None,
            wallet_transactions: Vec::new(),
        };
        let mut user_repo = UserRepoImpl::new(MemoryUserStorage::new());

        // Act
        let update_result = user_repo.update(&user);

        // Assert
        assert!(matches!(
            update_result.unwrap_err(),
            UserKvStorageError::UserNotFound { .. }
        ));
    }
}
