use super::error::{Result, WalletError};
use super::wallet::{TransactionIntent, WalletUser};
use crate::types::CryptoAmount;
use crate::types::{GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use async_trait::async_trait;
use iota_sdk::client::secret::SecretManager;
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk::types::block::payload::TaggedDataPayload;
use iota_sdk::types::block::payload::transaction::TransactionId;
use iota_sdk::wallet::ClientOptions;
use iota_sdk::wallet::account::{Account, SyncOptions, TransactionOptions};
use log::info;
use rust_decimal_macros::dec;
use std::fmt::Debug;
use std::path::Path;
use std::str::FromStr;

/// The number of addresses to automatically generate when setting up the wallet
const USER_ADDRESS_LIMIT: u32 = 20;

///The name of the application used as the account name in the wallet
const APP_NAME: &str = "standalone";

/// [`WalletUser`] implementation for IOTA and SMR using the stardust protocol
#[derive(Debug)]
pub struct WalletImplStardust {
    /// State for the account manager to be passed to calling functions
    account_manager: iota_sdk::wallet::Wallet,
}

impl WalletImplStardust {
    /// Creates a new [`WalletImpl`] from the specified [`Mnemonic`].
    pub async fn new(mnemonic: Mnemonic, path: &Path, coin_type: u32, node_url: &[String]) -> Result<Self> {
        // we now have the mnemonic and can initialize a wallet
        let node_urls: Vec<&str> = node_url.iter().map(String::as_str).collect();

        info!("Used node_urls: {:?}", node_urls);
        let client_options = ClientOptions::new()
            .with_local_pow(false)
            .with_fallback_to_local_pow(true)
            .with_nodes(&node_urls)?;

        // we need to make sure the path exists, or we will get IO errors, but only if we are not on wasm
        #[cfg(not(target_arch = "wasm32"))]
        if let Err(e) = std::fs::create_dir_all(path) {
            log::error!("Could not create the wallet directory: {e:?}");
        }

        let account_manager = {
            let secret_manager = SecretManager::try_from_mnemonic(mnemonic)?;
            iota_sdk::wallet::Wallet::builder()
                .with_client_options(client_options)
                .with_coin_type(coin_type)
                .with_secret_manager(secret_manager)
                .with_storage_path(path) // we still need this since the account stores stuff in jammdb
                .finish()
                .await?
        };

        let account = account_manager.get_account(APP_NAME).await;

        if let Err(account_error) = account {
            info!("{:?}", account_error);
            info!("Creating a new account with alias {APP_NAME}");
            account_manager
                .create_account()
                .with_alias(String::from(APP_NAME))
                .finish()
                .await?;
        }

        info!("Wallet creation successful");

        Ok(WalletImplStardust { account_manager })
    }

    async fn sync_wallet(&self) -> Result<()> {
        let account = self.account_manager.get_account(APP_NAME).await?;

        let options = SyncOptions {
            force_syncing: true,
            ..Default::default()
        };
        account.sync(Some(options)).await?;

        Ok(())
    }
}

/// The minimum amount of IOTA to consider an output as non-dust output. Everything below this is dust.
// SAFETY: the value is non-negative
const MIN_DUST_OUTPUT: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(0.1)) };

/// The value to divide an amount in GLOW with to get IOTA
// SAFETY: the value is non-negative
const GLOW_TO_IOTA_DIVISOR: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(1_000_000)) };

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletUser for WalletImplStardust {
    async fn get_address(&self) -> Result<String> {
        self.sync_wallet().await?;
        let account: Account = self.account_manager.get_account(APP_NAME).await?;
        let account_addresses = account.generate_ed25519_addresses(USER_ADDRESS_LIMIT, None).await?;
        let account_address = account_addresses.first();
        match account_address {
            Some(address) => {
                let bech32_address = address.clone().into_bech32().to_string();
                Ok(bech32_address)
            }
            None => Err(WalletError::EmptyWalletAddress),
        }
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        self.sync_wallet().await?;
        let account = self.account_manager.get_account(APP_NAME).await?;
        let balance = account.balance().await?;
        let available_balance_glow = balance.base_coin().available();
        let available_balance_iota = CryptoAmount::from(available_balance_glow) / GLOW_TO_IOTA_DIVISOR;
        Ok(available_balance_iota)
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        self.sync_wallet().await?;

        let TransactionIntent {
            address_to,
            amount,
            data,
        } = intent;

        let min_amount = *amount + MIN_DUST_OUTPUT;

        // Check if we have enough balance, otherwise return with err
        let balance = self.get_balance().await?;
        if balance < min_amount {
            return Err(WalletError::InsufficientBalance(String::from("Not enough balance")));
        }

        // hard-coded tag
        let tag: Box<[u8]> = "data".to_string().into_bytes().into_boxed_slice();
        let data: Box<[u8]> = data.to_owned().unwrap_or_default().into_boxed_slice();
        let tagged_data_payload = Some(TaggedDataPayload::new(tag, data).map_err(WalletError::Block)?);

        let account = self.account_manager.get_account(APP_NAME).await?;

        let options = TransactionOptions {
            allow_micro_amount: true,
            tagged_data_payload,
            ..Default::default()
        };

        let amount_glow: u64 = (amount.inner() * dec!(1_000_000)).round().try_into()?;
        let transaction = account.send(amount_glow, address_to, options).await?;

        let block_id = account
            .retry_transaction_until_included(&transaction.transaction_id, None, None)
            .await?;
        info!("Transaction successfully included in block: {block_id}!");

        Ok(transaction.transaction_id.to_string())
    }

    // Gets the list of transactions
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        let account = self.account_manager.get_account(APP_NAME).await?;

        // Calculate the start index for the current page
        let start_index = start * limit;

        // Fetch all iota transactions from the account
        // notice those are iota Transactions
        let all_transactions = account.transactions().await;

        // Extract transactions for the current page based on the start index and page size
        // and convert to the internal wallet Transaction
        let transactions_for_page = all_transactions
            .into_iter()
            .skip(start_index)
            .take(limit)
            .map(Into::into)
            .collect::<Vec<WalletTxInfo>>();

        Ok(WalletTxInfoList {
            transactions: transactions_for_page,
        })
    }

    /// Get detailed report of a particular transaction in the history
    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo> {
        let account = self.account_manager.get_account(APP_NAME).await?;
        // Parse the transaction ID string into the appropriate type, assuming TransactionId is a specific type
        let transaction_id: TransactionId = tx_id
            .parse()
            .map_err(|e: <TransactionId as FromStr>::Err| WalletError::InvalidTransaction(e.to_string()))?;

        // Fetch detailed transaction by its id using your wallet SDK's methods
        if let Some(transaction) = account.get_transaction(&transaction_id).await {
            Ok(transaction.into())
        } else {
            Err(WalletError::TransactionNotFound)
        }
    }

    async fn estimate_gas_cost(&self, _intent: &TransactionIntent) -> Result<GasCostEstimation> {
        // Stardust is fee-less
        Ok(GasCostEstimation {
            gas_limit: 0,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iota_sdk::crypto::keys::bip39::Mnemonic;
    use rstest::rstest;
    use rust_decimal_macros::dec;
    use testing::CleanUp;

    /// Mnemonic for testing.
    /// Iota: tst1qz7m7xtfppy9xd73xvsnpvlnx5rcewjz2k2gqh6w67tdleks83rh768k6rc
    pub const MNEMONIC: &str = "aware mirror sadness razor hurdle bus scout crisp close life science spy shell fine loop govern country strategy city soldier select diet brain return";

    const COIN_TYPE_IOTA: u32 = iota_sdk::client::constants::IOTA_COIN_TYPE;

    // General Note:
    // - Check the corresponding wallet address on the explorer: https://explorer.shimmer.network/testnet
    // - Obtain testnet tokens for the wallet (if needed): https://faucet.testnet.shimmer.network/

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user(mnemonic: impl Into<Mnemonic>, coin_type: u32) -> (WalletImplStardust, CleanUp) {
        let cleanup = testing::CleanUp::default();
        let wallet = WalletImplStardust::new(
            mnemonic.into(),
            Path::new(&cleanup.path_prefix),
            coin_type,
            &[String::from("https://api.testnet.iotaledger.net")],
        )
        .await
        .expect("should initialize wallet");
        (wallet, cleanup)
    }

    #[rstest]
    #[case(COIN_TYPE_IOTA, "tst")]
    #[tokio::test]
    async fn test_get_address(#[case] coin_type: u32, #[case] expected_prefix: &str) {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, coin_type).await;

        // Act
        let address = wallet_user.get_address().await;

        // Check the address prefix
        let address = address.unwrap(); // Unwrap the address since we already checked it's Ok
        assert!(
            address.starts_with(expected_prefix),
            "Address did not start with the expected prefix"
        );
    }

    #[rstest]
    #[case(COIN_TYPE_IOTA)]
    #[tokio::test]
    async fn test_get_balance(#[case] coin_type: u32) {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, coin_type).await;

        // Act
        let balance = wallet_user.get_balance().await;

        // Assert
        balance.unwrap();
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_serial_get_wallet_tx_list() {
        // Arrange
        let mnemonic = "west neutral cannon wreck notice disorder message three phrase accident office flavor merit kiss claim finish finger forum mesh mouse torch cradle inside glue";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, COIN_TYPE_IOTA).await;

        let address = wallet_user.get_address().await.unwrap(); // tst1qpaha27ytq8ay3ahqlfl6rn5xndnwxwahunf20h59cadpt9q2gx0crqekxk

        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();
        let index = String::from("TestMessageFromStandalone");

        let intent = TransactionIntent {
            address_to: address,
            amount,
            data: Some(index.into_bytes()),
        };

        let transaction = wallet_user.send_amount(&intent).await.unwrap();

        let start = 0;
        let limit = 10;

        // Act
        let result = wallet_user.get_wallet_tx_list(start, limit).await;

        // Assert
        let wallet_tx_info_list = result.unwrap();
        let transactions: Vec<crate::types::WalletTxInfo> = wallet_tx_info_list.transactions.to_vec();

        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].transaction_id, transaction);
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn test_serial_get_wallet_tx_success() {
        // Arrange
        let mnemonic = "century jazz giant zebra pledge head school supreme aim certain moment mechanic curtain chronic duck addict despair pistol romance risk impulse upgrade rubber grid";
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, COIN_TYPE_IOTA).await;

        let address = wallet_user.get_address().await.unwrap(); //tst1qq68c239vacsq3gnksqss5glh9e3w0wc9mra9d6cs39e8hs3xrmgjscdpac
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();
        let index = String::from("TestMessageFromStandalone");
        let intent = TransactionIntent {
            address_to: address,
            amount,
            data: Some(index.into_bytes()),
        };

        let transaction = wallet_user.send_amount(&intent).await.unwrap();

        // Act
        let result = wallet_user.get_wallet_tx(&transaction).await;

        // Assert
        assert_eq!(result.unwrap().transaction_id, transaction);
    }

    #[tokio::test]
    async fn test_get_wallet_tx_error_nonexistent_transaction() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, COIN_TYPE_IOTA).await;

        let transaction_id = "nonexistent_transaction_id";

        // Act
        let result = wallet_user.get_wallet_tx(transaction_id).await;

        // Assert
        assert!(result.is_err(), "Error occurred while fetching wallet transaction");
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn it_should_send_amount_with_data() {
        let mnemonic = "predict wrist plug desert mobile crowd build leg swap impose breeze loyal surge brand hair bronze melody scale hello cereal car item slow bring";
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(mnemonic, COIN_TYPE_IOTA).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");

        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();
        let message = Some(String::from("test message").into_bytes());
        let intent = TransactionIntent {
            address_to: address,
            amount,
            data: message,
        };

        // Act
        let result = wallet_user.send_amount(&intent).await;

        // Assert
        let transaction = result.unwrap();
        assert!(!transaction.is_empty());
    }

    #[cfg_attr(coverage, ignore = "Takes too long under code-coverage")]
    #[tokio::test]
    async fn it_should_send_amount_without_data() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, COIN_TYPE_IOTA).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(balance.inner() - dec!(1.0)).unwrap();
        let intent = TransactionIntent {
            address_to: address,
            amount,
            data: None,
        };

        // Act
        let result = wallet_user.send_amount(&intent).await;

        // Assert
        let transaction = result.unwrap();
        assert!(!transaction.is_empty());
    }

    #[tokio::test]
    async fn it_should_not_send_amount_more_then_balance() {
        // Arrange
        let (wallet_user, _cleanup) = get_wallet_user(MNEMONIC, COIN_TYPE_IOTA).await;

        let address = wallet_user.get_address().await.unwrap();
        let balance = wallet_user.get_balance().await.unwrap();
        println!("Address: {address}, balance: {balance:?}");
        let amount = CryptoAmount::try_from(dec!(96854.0)).unwrap();
        let intent = TransactionIntent {
            address_to: address,
            amount,
            data: None,
        };

        // Act
        let transaction = wallet_user.send_amount(&intent).await;

        // Assert
        assert!(
            matches!(transaction, Err(WalletError::InsufficientBalance(_))),
            "Got: {transaction:?}"
        );
    }
}
