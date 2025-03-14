use super::error::Result;
use super::wallet_user::WalletUser;
use crate::types::currencies::CryptoAmount;
use crate::types::transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use crate::wallet::error::WalletError;
use alloy::eips::BlockNumberOrTag;
use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use alloy::signers::local::coins_bip39::English;
use alloy::signers::local::MnemonicBuilder;
use alloy::{
    consensus::TxEip1559,
    primitives::Address,
    primitives::U256,
    providers::{Provider, ProviderBuilder},
};
use alloy_consensus::Transaction;
use alloy_primitives::TxHash;
use alloy_provider::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
};
use alloy_provider::{Identity, RootProvider, WalletProvider};
use async_trait::async_trait;
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk::wallet::account::types::InclusionState;
use log::info;
use reqwest::Url;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt::Debug;
use std::ops::{Div, Mul};
use std::str::FromStr;

const WEI_TO_ETH_DIVISOR: CryptoAmount = unsafe { CryptoAmount::new_unchecked(dec!(1_000_000_000_000_000_000)) }; // SAFETY: the value is non-negative

// Type alias for the crazy long type used as Provider with the default fillers (Gas, Nonce,
// ChainId) and Wallet
type ProviderType = FillProvider<
    JoinFill<
        JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

/// [`WalletUser`] implementation for ETH
#[derive(Debug)]
pub struct WalletImplEth {
    /// ChainId for the transactions.
    chain_id: u64,

    /// Rpc client, contains the Signer based on the mnemonic.
    provider: ProviderType,
}

impl WalletImplEth {
    /// Creates a new [`WalletImplEth`] from the specified [`Mnemonic`].
    pub async fn new(mnemonic: Mnemonic, node_urls: Vec<String>, chain_id: u64) -> Result<Self> {
        // Ase mnemonic to create a Signer
        // Child key at derivation path: m/44'/60'/0'/0/{index}.
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic.as_ref().to_string())
            .index(0)?
            // // Use this if your mnemonic is encrypted.
            // .password(password)
            .build()?;

        // construct the ProviderBuilder
        let url =
            Url::parse(&node_urls[0]).map_err(|e| WalletError::Parse(format!("could not parse the url: {e:?}")))?;

        // build a Provider that has the default fillers for GasEstimation, Nonce providing and chain_id fetcher
        let http_provider = ProviderBuilder::<_, _, Ethereum>::new()
            .wallet(wallet.clone())
            .on_http(url);

        info!("Wallet creation successful");

        Ok(WalletImplEth {
            chain_id,
            provider: http_provider,
        })
    }

    fn convert_wei_to_eth(value_wei: CryptoAmount) -> CryptoAmount {
        value_wei.div(WEI_TO_ETH_DIVISOR)
    }

    fn convert_eth_to_wei(value_eth: CryptoAmount) -> CryptoAmount {
        value_eth.mul(WEI_TO_ETH_DIVISOR)
    }

    fn convert_alloy_256_to_crypto_amount(v: alloy_primitives::Uint<256, 4>) -> Result<CryptoAmount> {
        let value_i128 = v.to::<i128>();
        let value_decimal = Decimal::from_i128(value_i128);
        match value_decimal {
            Some(result_decimal) => {
                let crypto_amount = CryptoAmount::try_from(result_decimal);
                match crypto_amount {
                    Ok(result_crypto_amount) => Ok(result_crypto_amount),
                    Err(_) => Err(WalletError::ConversionError(format!(
                        "could not convert decimal to crypto amount: {result_decimal:?}"
                    ))),
                }
            }
            None => Err(WalletError::ConversionError(format!(
                "could not convert alloy 256 to decimal: {v:?}"
            ))),
        }
    }

    fn convert_crypto_amount_to_u256(v: CryptoAmount) -> Result<alloy_primitives::U256> {
        let value_decimal = v.inner();
        let value_i128: i128 = value_decimal.try_into()?;
        let value = U256::from(value_i128);

        Ok(value)
    }
}

#[allow(unused_variables)] // allow unused parameters until everything is implemented
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(test, mockall::automock)]
impl WalletUser for WalletImplEth {
    async fn get_address(&self) -> Result<String> {
        Ok(self.provider.default_signer_address().to_string().to_lowercase())
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        let mut total = U256::ZERO;
        for addr in self.provider.signer_addresses() {
            let balance = self.provider.get_balance(addr).await?;
            log::info!("Balance for address {} = {}", addr, balance);
            total += balance;
        }

        let balance_wei_crypto_amount = Self::convert_alloy_256_to_crypto_amount(total)?;
        let balance_eth_crypto_amount = Self::convert_wei_to_eth(balance_wei_crypto_amount);
        Ok(balance_eth_crypto_amount)
    }

    async fn send_amount(&self, address: &str, amount: CryptoAmount, data: Option<Vec<u8>>) -> Result<String> {
        let addr_to = Address::from_str(address)?;
        let amount_wei = Self::convert_eth_to_wei(amount);
        let amount_wei_u256 = Self::convert_crypto_amount_to_u256(amount_wei)?;

        let mut tx = TransactionRequest::default()
            .with_to(addr_to)
            .with_chain_id(self.chain_id)
            .with_value(amount_wei_u256);

        // attach optional data to the transaction
        if let Some(data) = data {
            tx.set_input(data.to_owned());
        }

        // Send the transaction, the nonce is automatically managed by the provider.
        let pending_tx = self.provider.send_transaction(tx.clone()).await?;

        info!("Pending transaction... {}", pending_tx.tx_hash());

        // Wait for the transaction to be included and get the receipt.
        // Note: this might take some time so we should probably do it in the background in the future
        let receipt = pending_tx.get_receipt().await?;

        info!(
            "Transaction included in block {}",
            receipt.block_number.expect("Failed to get block number")
        );

        Ok(receipt.transaction_hash.to_string())
    }

    async fn send_transaction(&self, index: &str, address: &str, amount: CryptoAmount) -> Result<String> {
        unimplemented!("use send_transaction_eth");
    }

    async fn sync_wallet(&self) -> Result<()> {
        unimplemented!("dead interface");
    }

    async fn sync_transactions(&self, transactions: &mut [WalletTxInfo], start: usize, limit: usize) -> Result<()> {
        for i in start..start + limit {
            if let Some(transaction) = transactions.get_mut(i) {
                let synchronized_transaction = self.get_wallet_tx(&transaction.transaction_id).await;
                match synchronized_transaction {
                    Ok(stx) => *transaction = stx,
                    Err(e) => {
                        // On error, return historical (cached) transaction data
                        log::debug!(
                        "[sync_transactions] could not retrieve data about transaction from the network, transaction: {:?}, error: {:?}",
                        transaction.clone(), e
                        );
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    // The network does not provide information about historical transactions
    // (they can be retrieved manually, but this is a time-consuming process),
    // so the handling of this method is implemented at the SDK level.
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn get_wallet_tx(&self, transaction_id: &str) -> Result<WalletTxInfo> {
        let transaction_hash = TxHash::from_str(transaction_id)?;
        let transaction = self.provider.get_transaction_by_hash(transaction_hash).await?;

        let Some(tx) = transaction else {
            return Err(WalletError::TransactionNotFound);
        };

        let date = if let Some(block_number) = tx.block_number {
            let block = self
                .provider
                .get_block_by_number(BlockNumberOrTag::Number(block_number))
                .await?;
            block.map(|b| b.header.timestamp)
        } else {
            None
        };

        let my_address = self.get_address().await?;

        let is_transaction_incoming = tx.to().is_some_and(|to| to.to_string() == my_address);
        let value = tx.value();

        let receipt = self.provider.get_transaction_receipt(transaction_hash).await?;
        let status = match receipt.map(|r| r.inner.is_success()) {
            Some(true) => InclusionState::Confirmed,
            Some(false) => InclusionState::Conflicting,
            None => InclusionState::Pending,
        };

        let receipt = self.provider.get_transaction_receipt(transaction_hash).await?;
        let balance_wei_crypto_amount = Self::convert_alloy_256_to_crypto_amount(tx.value())?;
        let value_eth_crypto_amount = Self::convert_wei_to_eth(balance_wei_crypto_amount);

        let value_eth_f64: f64 = value_eth_crypto_amount.inner().try_into()?; // TODO: WalletTxInfo f64 -> Decimal ? maybe

        Ok(WalletTxInfo {
            date: date.map(|n| n.to_string()).unwrap_or_else(String::new),
            block_id: tx.block_number.map(|n| n.to_string()),
            transaction_id: transaction_id.to_string(),
            incoming: is_transaction_incoming,
            amount: value_eth_f64,
            network: "ETH".to_string(),
            status: format!("{:?}", status),
            explorer_url: None,
        })
    }

    async fn estimate_gas_cost_eip1559(&self, transaction: TxEip1559) -> Result<GasCostEstimation> {
        let from = self.get_address().await?;

        let to = transaction
            .to
            .to()
            .ok_or(WalletError::InvalidTransaction(String::from("receiver is empty")))?;

        let tx = TransactionRequest::default()
            .from(Address::from_str(&from)?)
            .to(*to)
            .input(TransactionInput::new(transaction.input))
            .access_list(transaction.access_list)
            .value(transaction.value);

        // Returns the estimated gas cost for the underlying transaction to be executed
        let gas_limit = self.provider.estimate_gas(tx).await?;

        // Estimates the EIP1559 `maxFeePerGas` and `maxPriorityFeePerGas` fields in wei.
        let eip1559_estimation = self.provider.estimate_eip1559_fees().await?;

        let max_priority_fee_per_gas = eip1559_estimation.max_priority_fee_per_gas;
        let max_fee_per_gas = eip1559_estimation.max_fee_per_gas;

        Ok(GasCostEstimation {
            max_fee_per_gas,
            max_priority_fee_per_gas,
            gas_limit,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;
    use alloy::primitives::Address;
    use iota_sdk::crypto::keys::bip39::Mnemonic;
    use rust_decimal::prelude::FromPrimitive;
    use serde_json::json;
    use testing::CleanUp;

    // Account #0: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (10000 ETH)
    // Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    pub const HARDHAT_MNEMONIC: &str = "test test test test test test test test test test test junk";

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user(mnemonic: impl Into<Mnemonic>) -> (WalletImplEth, CleanUp) {
        let (_, cleanup) = Config::new_test_with_cleanup();
        let node_url = vec![String::from("https://sepolia.mode.network")];
        let chain_id = 31337;

        let wallet = WalletImplEth::new(mnemonic.into(), node_url, chain_id)
            .await
            .expect("should initialize wallet");
        (wallet, cleanup)
    }

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user_with_mocked_provider(
        mnemonic: impl Into<Mnemonic>,
        node_url: String,
        chain_id: u64,
    ) -> WalletImplEth {
        // let (_, cleanup) = Config::new_test_with_cleanup();
        let wallet = WalletImplEth::new(mnemonic.into(), vec![node_url], chain_id)
            .await
            .expect("could not initialize WalletImplEth");

        wallet
    }

    #[tokio::test]
    async fn test_get_address() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        // Act
        let addr_raw = wallet_user.get_address().await.unwrap();

        // Assert
        assert!(addr_raw.starts_with("0x"));
    }

    #[tokio::test]
    async fn test_convert_wei_to_decimal() {
        //Arrange
        let value_wei: i128 = 10000000000000000000000; // 22 zeros
        let value_wei_decimal = Decimal::from_i128(value_wei).unwrap();
        // SAFETY: the value is non-negative
        let value_wei_crypto_amount: CryptoAmount = unsafe { CryptoAmount::new_unchecked(value_wei_decimal) };

        // Act
        let value_eth_crypto_amount = WalletImplEth::convert_wei_to_eth(value_wei_crypto_amount);

        // Assert

        /***
         * 1 10^22
         * 10 10^21
         * ...
         * 10000 10^18
         */
        assert_eq!(value_eth_crypto_amount, CryptoAmount::from(10000))
    }

    #[tokio::test]
    async fn test_convert_alloy_256_to_decimal() {
        //Arrange
        let expected_value_decimal: i128 = 10000000000000000000000;
        let expected_value_decimal = Decimal::from_i128(expected_value_decimal).unwrap();
        // SAFETY: the value is non-negative
        let expected_value_crypto_amount: CryptoAmount = unsafe { CryptoAmount::new_unchecked(expected_value_decimal) };

        let value_alloy_256: alloy_primitives::Uint<256, 4> =
            alloy_primitives::Uint::from_str("10000000000000000000000").unwrap();

        // Act
        let value_crypto_amount = WalletImplEth::convert_alloy_256_to_crypto_amount(value_alloy_256).unwrap();

        // Assert
        assert_eq!(value_crypto_amount, expected_value_crypto_amount)
    }

    #[tokio::test]
    async fn test_convert_crypto_amount_to_alloy_256() {
        //Arrange
        let expected_value_u256 = U256::from(1);
        let value_crypto_amount = CryptoAmount::from(1);

        // Act
        let value_u256 = WalletImplEth::convert_crypto_amount_to_u256(value_crypto_amount).unwrap();

        // Assert
        assert_eq!(value_u256, expected_value_u256)
    }

    #[tokio::test]
    async fn test_get_balance() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let wallet_addr = wallet_user.get_address().await.unwrap();

        let mocked_balance: i128 = 10_000_000_000_000_000_000_000; // 10^22
        let mocked_balance_in_hex = format!("0x{:X}", mocked_balance);

        let mocked_rpc_get_balance = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getBalance",
                "params": [
                    wallet_addr,
                    "latest"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": "{}"
                }}"#,
                mocked_balance_in_hex
            ))
            .create();

        // Act
        let balance = wallet_user.get_balance().await.unwrap();

        mocked_rpc_get_balance.assert();

        // Assert
        assert_eq!(balance, CryptoAmount::from(10000))
    }

    #[tokio::test]
    async fn test_send_amount_eth() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let mocked_transaction_count = 5;
        let amount_to_send = CryptoAmount::from(100);
        let from = wallet_user.get_address().await.unwrap();
        let to = String::from("0xb0b0000000000000000000000000000000000000");

        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "eth_estimateGas",
                "params": [{"from": from,"to":to,"value":"0x56bc75e2d63100000","input":"0x74657374206d657373616765", "chainId": "0x7a69"},"pending"],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "id": 0,
                    "result": 24009
                }"#,
            )
            .create();

        let mocked_rpc_eth_fee_history = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 1,
                "method": "eth_feeHistory",
            })))
            .with_status(200)
            .with_body(
                r#"{
                "jsonrpc": "2.0",
                "id": 1,
                "result": {
                    "baseFeePerGas": [1000000000, 875000000],
                    "gasUsedRatio": [0.0],
                    "oldestBlock": 0,
                    "reward": [[0]]
                }
            }
            "#,
            )
            .create();

        let mocked_rpc_get_transaction_count = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 2,
                "method": "eth_getTransactionCount",
                "params": [
                    from,
                    "pending"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 2,
                    "result": "{}"
                }}"#,
                mocked_transaction_count
            ))
            .create();

        let mocked_transaction_hash = "0x969dc1d6a97464e62fb1dab451b03d24111c278bf6f4d2e2b3910205a8682ed2";
        let mocked_rpc_send_raw_transaction = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_sendRawTransaction",
                "id": 3,
                "params": [
                    "0x02f87d827a6905018477359401825dc994b0b000000000000000000000000000000000000089056bc75e2d631000008c74657374206d657373616765c080a011114978927798fee734d1f11ad8b9b985755fa60f4036aa6320c08fa897372aa0291cb036983e0bcd059aa667d78904e6484e13b401f8c35cb7c125e6be947157"
                ],
            })))
            .with_status(200)
            .with_body(format!(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 3,
                    "result": "{}"
                }}"#,
                mocked_transaction_hash
            ))
            .create();

        let mocked_rpc_block_number = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_blockNumber",
                "id": 4,
                "params": [ ],
            })))
            .with_status(200)
            .with_body(
                r#"{{
                    "jsonrpc": "2.0",
                    "id": 4,
                    "result": "0x1505e9c"
                }}"#,
            )
            .create();

        let mocked_rpc_get_transaction_receipt_response_json = json!({
            "jsonrpc": "2.0",
            "id": 5,
            "result": {
                "transactionHash": mocked_transaction_hash,
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": "0x107d7b0",
                "contractAddress": null,
                "cumulativeGasUsed": "0x19aac9a",
                "effectiveGasPrice": "0xb9029a7ea",
                "from": from,
                "gasUsed": "0x27fb4",
                "logs": [],
                "logsBloom": "0x00000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000010000000000800020000000000000200000000040000000000800104008000000000000000000000800000000001200000000000000000000000000000000000000000000000020000000020010000800000000000000000000000000000000000000000000000000000000000000120000020800000000000000100084000000000000000000000000000000000000000000000002000000000000001000000000400000000000000000000000000000000010000000000000000000000000000000000000000000000000000090000000",
                "status": "0x1",
                "to": to,
                "transactionIndex": "0xfc",
                "type": "0x2"
            }
        });

        let mocked_rpc_get_receipt = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionReceipt",
                "params": [ mocked_transaction_hash ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_receipt_response_json).unwrap())
            .expect(2)
            .create();

        let metadata = String::from("test message").into_bytes();

        // Act
        let transaction_id = wallet_user.send_amount(&to, amount_to_send, Some(metadata)).await;

        // Assert
        mocked_rpc_eth_fee_history.assert();
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_get_transaction_count.assert();
        mocked_rpc_send_raw_transaction.assert();
        mocked_rpc_block_number.assert();
        mocked_rpc_get_receipt.assert();
        transaction_id.unwrap();
    }

    #[tokio::test]
    async fn test_get_wallet_tx_returns_error_when_transaction_cannot_be_found() {
        //Arrange
        let non_existent_transaction_hash = "0x00000000000000000000000000000000000000000000000000000000000e4404";

        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let mocked_rpc_get_transaction_by_hash = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionByHash",
                "params": [
                    non_existent_transaction_hash,
                ],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "id": 1,
                    "result": null
                }"#,
            )
            .create();

        // Act
        let transaction = wallet_user.get_wallet_tx(non_existent_transaction_hash).await;

        // Assert
        mocked_rpc_get_transaction_by_hash.assert();
        let error = transaction.unwrap_err();
        assert!(matches!(error, WalletError::TransactionNotFound))
    }

    #[tokio::test]
    async fn test_get_wallet_tx() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let dummy_transaction_hash = "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c";
        let dummy_block_number = "0x107d7b0";

        let mocked_rpc_get_transaction_by_hash_response_json = json!({
            "id": "0",
            "jsonrpc": "2.0",
            "result": {
                "accessList": [],
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": dummy_block_number,
                "chainId": "0x1",
                "from": "0x901c7c311d39e0b26257219765e71e8db3107a81",
                "gas": "0x31d74",
                "gasPrice": "0xb9029a7ea",
                "hash": dummy_transaction_hash,
                "input": "0x3593564c000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000646701fb000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000a968163f0a57b400000000000000000000000000000000000000000000000000000000000010326d79400000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000000002bb7135877cd5d40aa3b086ac6f21c51bbafbbb41f002710dac17f958d2ee523a2206206994597c13d831ec7000000000000000000000000000000000000000000",
                "maxFeePerGas": "0xf22a22912",
                "maxPriorityFeePerGas": "0x5f5e100",
                "nonce": "0x4",
                "r": "0xef566fc229bb0a10eee5f99c9cabe47f0f20ebaa6d16e4f7b90ee144086b21e9",
                "s": "0x109de5d9baca8daeee1ce1b7d1a304e223d07b1420b37704e675ccffd364a4dc",
                "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                "transactionIndex": "0xfc",
                "type": "0x2",
                "v": "0x0",
                "value": "0x0"
            }
        });

        let mocked_rpc_get_transaction_by_hash = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "eth_getTransactionByHash",
                "params": [ dummy_transaction_hash, ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_by_hash_response_json).unwrap())
            .create();

        let mocked_rpc_get_block_by_number_response_json = json!({
            "id": "1",
            "jsonrpc": "2.0",
            "result": {
                "difficulty": "0x1046bb7e3f8",
                "extraData": "0x476574682f76312e302e302d30636463373634372f6c696e75782f676f312e34",
                "gasLimit": "0x1388",
                "gasUsed": "0x0",
                "hash": "0xf7756d836b6716aaeffc2139c032752ba5acf02fe94acb65743f0d177554b2e2",
                "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
                "miner": "0x33bc13fdf135073277971b4d9f4f72082e907996",
                "mixHash": "0x8c2dc0f970fa3aa6beb64c9f06a202a4314acfa4effaa4c75fd5bc9f9c77a519",
                "nonce": "0x28df43dd283aab1d",
                "number": "0x107d7b0",
                "parentHash": "0xbc33aa8829350cc2e3ba7cf64d4beb2f1b554d570efc8bccb7b05ef50d76a47a",
                "receiptsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
                "size": "0x223",
                "stateRoot": "0x8af5429b649f9fc633ce3c95219026fd08a249867e28c7eab22994eaa6125bb9",
                "timestamp": "0x55bf47e3",
                "totalDifficulty": "0x3f3cfd84833af0",
                "transactions": [],
                "transactionsRoot": "0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421",
                "uncles": []
            }
        });

        let mocked_rpc_get_block_by_number = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getBlockByNumber",
                "id": 1,
                "params": [ dummy_block_number ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_block_by_number_response_json).unwrap())
            .create();

        let mocked_rpc_get_transaction_receipt_response_json = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "blockHash": "0xe6262c1924326d12b88aaa35a95a0c7cdd11f2d20ebae84618484120bd037c34",
                "blockNumber": "0x107d7b0",
                "contractAddress": null,
                "cumulativeGasUsed": "0x19aac9a",
                "effectiveGasPrice": "0xb9029a7ea",
                "from": "0x901c7c311d39e0b26257219765e71e8db3107a81",
                "gasUsed": "0x27fb4",
                "logs": [],
                "logsBloom": "0x00000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000010000000000800020000000000000200000000040000000000800104008000000000000000000000800000000001200000000000000000000000000000000000000000000000020000000020010000800000000000000000000000000000000000000000000000000000000000000120000020800000000000000100084000000000000000000000000000000000000000000000002000000000000001000000000400000000000000000000000000000000010000000000000000000000000000000000000000000000000000090000000",
                "status": "0x1",
                "to": "0xef1c6e67703c7bd7107eed8303fbe6ec2554bf6b",
                "transactionHash": "0xcd718a69d478340dc28fdf6bf8056374a52dc95841b44083163ced8dfe29310c",
                "transactionIndex": "0xfc",
                "type": "0x2"
            }
        });

        let mocked_rpc_get_transaction_receipt = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_getTransactionReceipt",
                "params": [ dummy_transaction_hash ],
            })))
            .with_status(200)
            .with_body(serde_json::to_vec(&mocked_rpc_get_transaction_receipt_response_json).unwrap())
            .expect(2)
            .create();

        // Act
        let transaction = wallet_user.get_wallet_tx(dummy_transaction_hash).await;

        // Assert
        mocked_rpc_get_transaction_by_hash.assert();
        mocked_rpc_get_transaction_receipt.assert();
        mocked_rpc_get_block_by_number.assert();
        transaction.unwrap();
    }

    #[tokio::test]
    async fn should_estimate_gas_cost_eip1559() {
        // Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let to = String::from("0xb0b0000000000000000000000000000000000000");
        let value = U256::from(1);
        let chain_id = 31337;
        let transaction_data = "data";

        let tx = TxEip1559 {
            chain_id,
            nonce: 0,
            gas_limit: 0,
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            to: alloy_primitives::TxKind::Call(Address::from_str(&to).unwrap()),
            value,
            access_list: Default::default(),
            input: alloy::hex::encode(transaction_data).into(),
        };

        let expected_estimation = GasCostEstimation {
            gas_limit: 24009,
            max_fee_per_gas: 2000000001,
            max_priority_fee_per_gas: 1,
        };

        let from = wallet_user.get_address().await.unwrap();
        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "eth_estimateGas",
                "params": [{"from": from,"to": to,"value": "0x1"},"pending"],
            })))
            .with_status(200)
            .with_body(
                r#"{
                    "jsonrpc": "2.0",
                    "result": 24009
                }"#,
            )
            .create();

        let mocked_rpc_eth_fee_history = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "method": "eth_feeHistory",
            })))
            .with_status(200)
            .with_body(
                r#"{
                "jsonrpc": "2.0",
                "result": {
                    "baseFeePerGas": [1000000000, 875000000],
                    "gasUsedRatio": [0.0],
                    "oldestBlock": 0,
                    "reward": [[0]]
                }
            }
            "#,
            )
            .create();

        // Act
        let result = wallet_user.estimate_gas_cost_eip1559(tx).await;

        // Assert
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_eth_fee_history.assert();
        let response = result.unwrap();

        assert_eq!(expected_estimation, response)
    }
}
