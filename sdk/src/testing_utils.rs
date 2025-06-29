#![allow(clippy::unwrap_used)]

use crate::tx_version::VersionedWalletTransaction;
use crate::types::users::{KycType, UserEntity};
use crate::types::viviswap::{
    ViviswapAddressDetail, ViviswapPartiallyKycDetails, ViviswapState, ViviswapVerificationStatus,
    ViviswapVerificationStep,
};
use crate::wallet_manager::MockWalletManager;
use crate::{
    core::Config,
    types::newtypes::{AccessToken, EncryptedPassword, EncryptionPin, PlainPassword},
    user::MockUserRepo,
    wallet_manager::WalletBorrow,
};
use api_types::api::dlt::{Course, GetCourseResponse};
use api_types::api::networks::{ApiNetwork, ApiProtocol};
use api_types::api::{
    postident::{CaseDetailsResponse, NewCaseIdResponse},
    transactions::{ApiApplicationMetadata, ApiTxStatus, GetTransactionDetailsResponse},
    viviswap::{
        contract::{
            ViviswapApiContractBankDetails, ViviswapApiContractCryptoDetails, ViviswapApiContractDetails,
            ViviswapContract, ViviswapContractCreationResponse,
        },
        detail::{GetPaymentDetailsResponse, PaymentDetail, SwapPaymentDetailKey},
        order::Order,
        payment::{ViviPaymentMethod, ViviPaymentMethodsResponse},
    },
};
use chrono::{DateTime, Utc};
use etopay_wallet::MockWalletUser;
use etopay_wallet::types::WalletTransaction;
use etopay_wallet::types::{CryptoAmount, WalletTxStatus};
use mockito::{Server, ServerOpts};
use rust_decimal_macros::dec;
use std::sync::LazyLock;
use testing::CleanUp;

pub const USERNAME: &str = "test_user";
pub const HEADER_X_APP_NAME: &str = "X-APP-NAME";
pub const AUTH_PROVIDER: &str = "standalone";
pub const ADDRESS: &str = "smrq1..";
pub static TOKEN: LazyLock<AccessToken> = LazyLock::new(|| AccessToken::try_from_string("test_token").unwrap());
pub const SERVER_OPTS: ServerOpts = mockito::ServerOpts {
    assert_on_drop: true,
    host: "127.0.0.1",
    port: 0,
};
pub const ENCRYPTED_SHARE: &str = "ME-RS-AesGcm-K3vx+e6IF6BOUJ2DemvsdflQq2CbolcFqdazfapauZTHdY/Hovh5zC8s5Qmfb2tRmRaluRX1gxMZfGDP52rakFnZpOzOCNGyHiI/dsiFDFbty0fEheEw+p1LrOI4zNwy7NE7ZsK0C756ggVfrhCin2Yw0KA6pALFqfWnQokx5Q43pUFd6ZGD8fwathC4NGx/hVTi9lxA2L6ScNQY9V3bEie40MKdpLQ6ELsPq+38UVJtqIgE0wJs8fDKSIGJVEPvP6wbVa+oPB/uFl5h56YeuYB2UGHdMJ54DCEoUBSd5QGoeKwjIylrZ+wXzchPXhtAfaCmqlf0fmKi9f5FQGrFwH9drf5HFE5Z/JWQC1FMKJTeBZ2CgcFvCtHuVm8VNnhhes1fUc7gL8VNOqE25LHFFQp3fBfeHXRkCmX+PAU+1N8KU6SFX0XqDr5anKAMH6thViBdno2m6K9tzqyucUnfgHYgp/cc+XXo9Ffw7v6lVTW3ls9diZwdwcs9JYqoKhWAs9dVGPz0017glpeAz01moJDPSMkhZwQh9GGWvhyeTWE9T28NS1G3cOBkW0GbgmIDjKeDDXAOjDyN7Db0FFL3TRAXthFtRXjJyZD1Xu2quYyjz1ZG70ILp0rDzzDaikUPUt1TCsAz+8NfLwHKz+H4oPUGprdUqgBVSGOySH+lKZaUbN17qIXjEKg58jh686s6i4GTD7Ndf6Xqsdc00PRDlm+jHwK7bNvkqkcChQHockIaIi4ETHCz/jqrca7uY8RIABv9Ni46+Ix1CrNY4qCUhep9oYZBGSLy2fQWWNk2nZgbrkipwUbgoV1IJV/kWCQ6ycjGG005kv3AFb6sZyrnFbvT7sa/JCKlo8gcVtzXlrJJqiO/7Qb1nTfj9dLd+/4ihpmwpFwPmKHi6zrZjJ8FbaDGkXSg+a82RQqz/AsH10hBd/tSZeZ5chdwgxTouoGix99HZipTKXLiAqW7Mo0N93+atNb9EWeHPBfsVbwJ2shBT5030QrY2qQfhTb4GUl52vPQBvpjxCzjPlvCWzFMlO8wrCP1sJm5egEb0F6Fpa9H3blBdMcb2NuKJ2VfSQzuJrbLzirnX3X0Pbk93S2dE5vs/2xsL6fqV18EPkVXO1mQtqsM8sMF8o6G/PLILN268Ga7CwcCL3qnoaCvahN3sHbciy38UH6s5hRTDvV75nWDj4oIaByrYx+JdgSZ4sucAn/bEQJCDSTVQ3sYQbEJGxc+xImNWudEoxdCmKYZPDFhUEIfO8pQRHTX8ZHZST+m97kJMuPvWg49UlrGu2YE6KbkNBEz7cSoWOuWpbrNjv1I8XKf8Sd82dvRWn3ZDc/4GXXE5oscG8UHTlz3XIpWNNrpE+wmn+AvmU0+n5r4Nv0LOFrlqH8Z2DcfjGqAVJkQMWFriruEcsPOvRgvGUeUtjulxEwcqX/UVmE5871rx0C2aJhazTnLkzt9TDFTaAf7J7zkIkhvKx8AU2A=";
pub const NOT_ENCRYPTED_SHARE: &str = "ME-RS-N-Mi0xLUNBRVFBaGdESXFBRStPVUZYZTJnMTdLRFY1L2pWRllQTHdtZ0dCWExJbitjTERReFRyRHArWGNVMG5yY3UyVmFONFEvZkVoeXNadm5qNFhmRDVIZXZ3eHB2bENTYnZIZTFtOTlXdjJwby8zVWl0d2VhMnVWOTZaejB5WmhEdHlkRDFYcEg1R0RIYXFvZDBpTHdpcDZ3d1k5T0VWdEJhZmtkUVRGaTNNM3gvY2dsK0FDWVQ5WG50TlJycnRtWFRTUGZ4MG54R1lVc0NWUnNKY3h5Q0JxSHBlRGVRekpSTlFxVldMNGpJU3JCZkFRcEpYMnJoT1o4OXM1V3VLaW5PWFd0YUZncTRnd2t1VzR0ZkJJZzVUMjFlaXpGNEpWNzlMcXFXSDZoY3N0Z1huYzZYWTJvZjRvaytlYnJWOFBmR1lOU1NxRWQ4VFpqUzlBL0h0clJGNThEbUdaL2Z2Nmp5MjJjS01hUWllK1ZqdFZ4OUJyblJjWThYYTgxWmNTWlF4YlFLbFQ3MC9tRk5aQlN4ZXNLTWVTU24vV2hycEs0OU80ZW4zRkZJVTJqd2lLcGwybHpHMk0vdThJTzRZSlNCL1B6aVp4cGczcVk5Z25PRHNQR2lDZGNyejErcTVhYUdoMDdXUGlISFg5K1VpbVJjRThZS1BBNXUwNTBkQ2l2eVM2a2VhZkpFalQ0UXkxcElPaFRUd3ZrMWxrR0ZmeWp3bVBqL3JMRGY4YUc3ZXZlVWQveGwxbzlKMnh5ckhvQW9heTNVNVpHYjFCZGJ2OGFGNHJLb2wwTkorUlZBSTZJSHJCUnE4OGxJeGtzSlFxTm9GQ3o5b051N011OTVkMUJpZ3ErNjZiYzBuTWcyWXZYQXdaMkh3RjAzS0xRWEFWYjZVekZ1Lzc0MjYraElNUlR1M01mZDZoa01vMllMVzlxSS9odlBsaWg4RG5qaUFTUG9Fbkx2cVFidVpXaVBnQ3h2c1F4eXFBQWdDazlzckhwaG51UnpTck95M1JmZzRYa0lndHhlb3ArUmZJOVgyaDBRcEVmcjgzYzExd0xhQkxDUmgwMlFXazA2Ty8yM2s2cWZNZHBxNVZ2b0ZnTkNJYlY1V01sSFpaV3RnVXFzaGtXRVJycjduZnVvd1BQQ0NUaHdxMC9tbUQ1NDVDb0VNWU16bUtQYlIyYmF4RkVTbUswTlRRT3VWR3A2Y3JqNWlYOGxzaU9kZ3FVNHhuSVpRcDRsT1lJcTlBOUhFS3NZZ1RuYysxRlRNazJEN05ydThlalh4UUR3amFqUTFNTmJ5cldBS0MvZ3RTWW9ONTFKY25FWFlUOWI1MVZOWWF4anArTE9oeDA0M3RNUW9TejNvN1kxbWtORlJUTmJMZWhDKzV0UkNKNjdQYk5Va29DbWxXbjFYODZxVlVsRFU0MjkxaXVLaE1YQ0lsVHlscGU2dw==";
pub const TX_INDEX: &str = "deadbeef-1234-2341-6afe";
pub const PRODUCT_HASH: &str = "hash";
// SAFETY: we know that this value is not negative
pub const AMOUNT: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(1_000)) };
pub const REASON: &str = "COMPLIMENT";
pub const PURCHASE_MODEL: &str = "CLIK";
pub const RECEIVER: &str = "satoshi";
pub const APP_DATA: &str = "app_data";
pub const START: u32 = 1;
pub const LIMIT: u32 = 10;
pub const PAYMENT_METHOD_KEY: SwapPaymentDetailKey = SwapPaymentDetailKey::Iota;
pub const PAYMENT_METHOD_KEY_SERIALIZED: &str = "IOTA";
pub const PAYMENT_METHOD_ID: &str = "payment-method-id";
pub const PAYMENT_DETAIL_ID: &str = "payment-detail-id";
pub const CASE_ID: &str = "123";

pub static ENCRYPTED_WALLET_PASSWORD: LazyLock<EncryptedPassword> = LazyLock::new(|| {
    // SAFETY: The byte array was generated and validated at compile time
    unsafe {
        EncryptedPassword::new_unchecked([
            91, 105, 102, 64, 68, 93, 59, 70, 41, 198, 141, 36, 152, 135, 67, 77, 191, 58, 227, 216, 53, 74, 74, 1,
            168, 243, 227, 153, 73, 141, 159, 91, 79, 39, 208, 59, 54, 101, 112, 107, 169,
        ])
    }
});
pub static WALLET_PASSWORD: LazyLock<PlainPassword> =
    LazyLock::new(|| PlainPassword::try_from_string("correcthorsebatterystaple").unwrap());
pub static PIN: LazyLock<EncryptionPin> = LazyLock::new(|| EncryptionPin::try_from_string("123456").unwrap());
pub const SALT: [u8; 12] = [241, 167, 131, 245, 166, 203, 63, 247, 211, 157, 138, 34];

pub const PURCHASE_ID: &str = "123";
pub const ORDER_ID: &str = "497f6eca-6276-4993-bfeb-53cbbbba6f08";

pub const IOTA_NETWORK_KEY: &str = "IOTA";
pub const ETH_NETWORK_KEY: &str = "ETH";

/// Mnemonic for testing.
/// Iota: tst1qz7m7xtfppy9xd73xvsnpvlnx5rcewjz2k2gqh6w67tdleks83rh768k6rc
pub const MNEMONIC: &str = "aware mirror sadness razor hurdle bus scout crisp close life science spy shell fine loop govern country strategy city soldier select diet brain return";

// util function to set the config
pub async fn set_config() -> (Server, Config, CleanUp) {
    let srv = mockito::Server::new_with_opts_async(SERVER_OPTS).await;
    let path = "/api";
    let url = format!("{}{}", srv.url(), path);

    let (config, cleanup) = Config::new_test_with_cleanup_url(&url);
    (srv, config, cleanup)
}

pub fn example_get_user(key: SwapPaymentDetailKey, verified: bool, times: usize, kyc_type: KycType) -> MockUserRepo {
    let mut mock_user_repo = MockUserRepo::new();
    mock_user_repo.expect_get().times(times).returning(move |r1| {
        assert_eq!(r1, USERNAME);
        Ok(UserEntity {
            user_id: None,
            username: USERNAME.to_string(),
            encrypted_password: Some(ENCRYPTED_WALLET_PASSWORD.clone()),
            salt: SALT.into(),
            is_kyc_verified: verified,
            kyc_type: kyc_type.to_owned(),
            viviswap_state: Some(ViviswapState {
                verification_status: ViviswapVerificationStatus::Verified,
                monthly_limit_eur: 250.000,
                next_verification_step: ViviswapVerificationStep::Documents,
                partial_kyc_details_input: ViviswapPartiallyKycDetails::new(),
                current_iban: Some(ViviswapAddressDetail {
                    id: "some id".to_string(),
                    address: ADDRESS.to_string(),
                    is_verified: true,
                }),
                payment_methods: Some(ViviPaymentMethodsResponse {
                    methods: Vec::from([
                        ViviPaymentMethod {
                            id: "497f6eca-6276-4993-bfeb-53cbbbba6f08".into(),
                            key: SwapPaymentDetailKey::Sepa,
                            min_amount: 1.5f32,
                            max_amount: 1000.4422f32,
                            supported_deposit_currencies: Vec::from(["IOTA".into()]),
                            supported_withdrawal_method_keys: Vec::from([SwapPaymentDetailKey::Sepa]),
                            contract_type: "Standard".into(),
                            is_incoming_payment_detail_required: true,
                            is_incoming_amount_required: true,
                            network_identifier: "sepa".to_string(),
                        },
                        ViviPaymentMethod {
                            id: "497f6eca-6276-4993-bfeb-53cbbbba6f08".into(),
                            key,
                            min_amount: 1.5f32,
                            max_amount: 1000.4422f32,
                            supported_deposit_currencies: Vec::from(["IOTA".into()]),
                            supported_withdrawal_method_keys: Vec::from([key]),
                            contract_type: "Standard".into(),
                            is_incoming_payment_detail_required: true,
                            is_incoming_amount_required: true,
                            network_identifier: format!("{:?}", key),
                        },
                    ]),
                }),
            }),
            local_share: None,
            wallet_transactions: Vec::new(),
            wallet_transactions_versioned: Vec::new(),
        })
    });
    mock_user_repo
}

pub fn example_tx_metadata() -> ApiApplicationMetadata {
    ApiApplicationMetadata {
        product_hash: PRODUCT_HASH.into(),
        reason: REASON.into(),
        purchase_model: PURCHASE_MODEL.into(),
        app_data: APP_DATA.into(),
    }
}

pub fn example_tx_details() -> GetTransactionDetailsResponse {
    GetTransactionDetailsResponse {
        system_address: ADDRESS.to_string(),
        amount: AMOUNT.inner().into(),
        status: ApiTxStatus::Completed,
        network: ApiNetwork {
            key: IOTA_NETWORK_KEY.to_string(),
            is_testnet: true,
            display_name: String::from("IOTA"),
            display_symbol: String::from("IOTA"),
            coin_type: 4218,
            node_urls: vec![String::from("https://api.testnet.iota.cafe")],
            decimals: 8,
            can_do_purchases: true,
            protocol: ApiProtocol::IotaRebased {
                coin_type: "0x2::iota::IOTA".to_string(),
            },
            block_explorer_url: String::from("https://iotascan.com/testnet"),
        },
    }
}

pub fn example_api_network(key: String) -> ApiNetwork {
    match key {
        val if val == *"IOTA" => ApiNetwork {
            key: IOTA_NETWORK_KEY.to_string(),
            is_testnet: true,
            display_name: String::from("IOTA"),
            display_symbol: String::from("IOTA"),
            coin_type: 4218,
            node_urls: vec![String::from("https://api.testnet.iota.cafe")],
            decimals: 8,
            can_do_purchases: true,
            protocol: ApiProtocol::IotaRebased {
                coin_type: "0x2::iota::IOTA".to_string(),
            },
            block_explorer_url: String::from("https://iotascan.com/testnet"),
        },
        val if val == *"ETH" => ApiNetwork {
            key: ETH_NETWORK_KEY.to_string(),
            is_testnet: true,
            display_name: String::from("Eth Sepolia"),
            display_symbol: String::from("ETH"),
            coin_type: 60,
            node_urls: vec![String::from("https://sepolia.mode.network")],
            decimals: 16,
            can_do_purchases: true,
            protocol: ApiProtocol::Evm { chain_id: 31337 },
            block_explorer_url: String::from("https://explorer.shimmer.network/testnet/"),
        },
        _ => unreachable!("nonexistant!"),
    }
}

pub fn example_api_networks() -> Vec<ApiNetwork> {
    vec![
        ApiNetwork {
            key: IOTA_NETWORK_KEY.to_string(),
            is_testnet: true,
            display_name: String::from("IOTA"),
            display_symbol: String::from("IOTA"),
            coin_type: 4218,
            node_urls: vec![String::from("https://api.testnet.iota.cafe")],
            decimals: 8,
            can_do_purchases: true,
            protocol: ApiProtocol::IotaRebased {
                coin_type: "0x2::iota::IOTA".to_string(),
            },
            block_explorer_url: String::from("https://iotascan.com/testnet"),
        },
        ApiNetwork {
            key: ETH_NETWORK_KEY.to_string(),
            is_testnet: true,
            display_name: String::from("Eth Sepolia"),
            display_symbol: String::from("ETH"),
            coin_type: 60,
            node_urls: vec![String::from("https://sepolia.mode.network")],
            decimals: 16,
            can_do_purchases: true,
            protocol: ApiProtocol::Evm { chain_id: 31337 },
            block_explorer_url: String::from("https://explorer.shimmer.network/testnet/"),
        },
    ]
}

pub fn example_new_case_id() -> NewCaseIdResponse {
    NewCaseIdResponse {
        case_id: "ABCDEFGH".into(),
        case_url: "https://example.org/start/new-case".into(),
    }
}

pub fn example_case_details() -> CaseDetailsResponse {
    CaseDetailsResponse {
        case_id: "ABCDEFGH".into(),
        archived: false,
        status: "Pending".into(),
    }
}

pub fn example_viviswap_oder_response() -> Order {
    Order {
        id: "order-id".into(),
        is_payed_out: true,
        is_approved: true,
        is_canceled: true,
        fees_amount_eur: 1.0f32,
        crypto_fees: 0.0f32,
        contract_id: "contract-id".into(),
        incoming_payment_method_id: PAYMENT_METHOD_ID.into(),
        incoming_payment_method_currency: "IOTA".to_string(),
        incoming_amount: 1.0f32,
        incoming_course: 1.0f32,
        outgoing_payment_method_id: PAYMENT_METHOD_ID.into(),
        outgoing_payment_method_currency: "EUR".into(),
        outgoing_amount: 1.0f32,
        outgoing_course: 1.0f32,
        refund_amount: None,
        refund_course: None,
        refund_payment_method_id: None,
        status: 0i32,
        creation_date: "2222-22-22".into(),
        incoming_payment_detail: None,
        outgoing_payment_detail: None,
        refund_payment_detail: None,
    }
}

pub fn example_get_payment_details_response() -> GetPaymentDetailsResponse {
    GetPaymentDetailsResponse {
        payment_detail: vec![PaymentDetail {
            id: PAYMENT_DETAIL_ID.into(),
            address: ADDRESS.into(),
            is_verified: Some(true),
        }],
    }
}

pub fn example_exchange_rate_response() -> GetCourseResponse {
    GetCourseResponse {
        course: Course {
            course: dec!(1.0).into(),
            date: "2222-22-22".into(),
        },
    }
}

pub fn example_contract_response(details: Option<ViviswapApiContractDetails>) -> ViviswapContractCreationResponse {
    ViviswapContractCreationResponse {
        contract: Some(ViviswapContract {
            id: "497f6eca-6276-4993-bfeb-53cbbbba6f08".to_string(),
            reference: "2c4a230c-5085-4924-a3e1-25fb4fc5965b".to_string(),
            incoming_payment_method_id: "6c21b3c4-8f92-4577-b887-11e932f12e12".to_string(),
            incoming_payment_detail_id: Some("28502a18-df98-4466-bf71-1fe8cc260cc2".to_string()),
            outgoing_payment_method_id: "a42cc08e-c976-4dff-bff4-b5ffc7f909ba".to_string(),
            outgoing_payment_detail_id: "77a12090-98ae-46d8-ae6a-8d6d502473cd".to_string(),
            details,
        }),
    }
}

pub fn example_bank_details() -> Option<ViviswapApiContractDetails> {
    Some(ViviswapApiContractDetails::BankAccount(
        ViviswapApiContractBankDetails {
            beneficiary: USERNAME.to_string(),
            name_of_bank: "DB".to_string(),
            address_of_bank: "some bank address".to_string(),
            address: "some user address".to_string(),
            bic: "bic".to_string(),
        },
    ))
}

pub fn example_crypto_details() -> Option<ViviswapApiContractDetails> {
    Some(ViviswapApiContractDetails::Crypto(ViviswapApiContractCryptoDetails {
        deposit_address: ADDRESS.into(),
        wallet_id: "some".into(),
    }))
}

pub fn example_wallet_borrow() -> MockWalletManager {
    let mut mock_wallet_manager = MockWalletManager::new();
    mock_wallet_manager.expect_try_get().returning(move |_, _, _, _, _, _| {
        let mock_wallet_user = MockWalletUser::new();
        Ok(WalletBorrow::from(mock_wallet_user))
    });
    mock_wallet_manager
}

pub fn example_versioned_wallet_transaction() -> VersionedWalletTransaction {
    let mock_date = DateTime::parse_from_rfc3339("2024-06-01T12:00:00Z")
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap();

    VersionedWalletTransaction::V2(WalletTransaction {
        date: mock_date,
        block_number_hash: None,
        transaction_hash: "some tx id".to_string(),
        receiver: String::new(),
        sender: String::new(),
        // SAFETY: the value is non-negative
        amount: unsafe { CryptoAmount::new_unchecked(dec!(20)) },
        network_key: "IOTA".to_string(),
        status: WalletTxStatus::Confirmed,
        explorer_url: None,
        gas_fee: None,
        is_sender: true,
    })
}
