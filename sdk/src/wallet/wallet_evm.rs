use super::error::Result;
use super::wallet::{TransactionIntent, WalletUser};
use crate::types::currencies::CryptoAmount;
use crate::types::transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use crate::wallet::error::WalletError;
use alloy::eips::BlockNumberOrTag;
use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder};
use alloy::rpc::types::TransactionRequest;
use alloy::signers::local::MnemonicBuilder;
use alloy::signers::local::coins_bip39::English;
use alloy::sol_types::SolCall;
use alloy::{
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
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use std::fmt::Debug;
use std::str::FromStr;

// Type alias for the crazy long type used as Provider with the default fillers (Gas, Nonce,
// ChainId) and Wallet
type ProviderType = FillProvider<
    JoinFill<
        JoinFill<Identity, JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;

/// [`WalletUser`] implementation for EVM
#[derive(Debug)]
pub struct WalletImplEvm {
    /// ChainId for the transactions.
    chain_id: u64,

    /// The number of decimals for the symbol value
    decimals: u32,

    /// Rpc client, contains the Signer based on the mnemonic.
    provider: ProviderType,
}

impl WalletImplEvm {
    /// Creates a new [`WalletImplEvm`] from the specified [`Mnemonic`].
    pub fn new(
        mnemonic: Mnemonic,
        node_urls: Vec<String>,
        chain_id: u64,
        decimals: u32,
        coin_type: u32,
    ) -> Result<Self> {
        // Ase mnemonic to create a Signer
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic.as_ref().to_string())
            // Child key at derivation path: m/44'/{coin_type}'/{account}'/{change}/{index}.
            .derivation_path(format!("m/44'/{}'/0'/0/0", coin_type))?
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

        Ok(WalletImplEvm {
            chain_id,
            decimals,
            provider: http_provider,
        })
    }

    /// Convert a [`U256`] to [`CryptoAmount`] while taking the decimals into account.
    fn convert_alloy_256_to_crypto_amount(&self, v: alloy_primitives::Uint<256, 4>) -> Result<CryptoAmount> {
        convert_alloy_256_to_crypto_amount(v, self.decimals)
    }

    /// Convert a [`CryptoAmount`] to [`U256`] while taking the decimals into account.
    fn convert_crypto_amount_to_u256(&self, v: CryptoAmount) -> Result<alloy_primitives::U256> {
        convert_crypto_amount_to_u256(v, self.decimals)
    }

    /// Helper function that prepares the [`TransactionRequest`] so that we can also use the same logic for gas estimation.
    fn prepare_transaction(&self, intent: &TransactionIntent) -> Result<TransactionRequest> {
        let TransactionIntent {
            address_to,
            amount,
            data,
        } = intent;

        let addr_to = Address::from_str(address_to)?;
        let amount_wei_u256 = self.convert_crypto_amount_to_u256(*amount)?;

        let mut tx = TransactionRequest::default()
            .with_to(addr_to)
            .with_chain_id(self.chain_id)
            .with_value(amount_wei_u256);

        // attach optional data to the transaction
        if let Some(data) = data {
            tx.set_input(data.to_owned());
        }

        Ok(tx)
    }

    /// Submit the [`TransactionRequest`] and wait for it to be included in a block.
    async fn submit_transaction_request(&self, tx_request: TransactionRequest) -> Result<String> {
        // Send the transaction, the nonce is automatically managed by the provider.
        let pending_tx = self.provider.send_transaction(tx_request).await?;

        info!("Pending transaction... {}", pending_tx.tx_hash());

        // Wait for the transaction to be included and get the receipt.
        // Note: this might take some time so we should probably do it in the background in the future
        let receipt = pending_tx.get_receipt().await?;

        info!("Transaction included in block {:?}", receipt.block_number);

        Ok(receipt.transaction_hash.to_string())
    }

    async fn estimate_transaction_request_gas(&self, tx_request: TransactionRequest) -> Result<GasCostEstimation> {
        // Returns the estimated gas cost for the underlying transaction to be executed
        let gas_limit = self.provider.estimate_gas(tx_request).await?;

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

/// Convert a [`U256`] to [`CryptoAmount`] while taking the decimals into account.
fn convert_alloy_256_to_crypto_amount(value: U256, decimals: u32) -> Result<CryptoAmount> {
    let value_u128 = u128::try_from(value)
        .map_err(|e| WalletError::ConversionError(format!("could not convert U256 to u128: {e:?}")))?;

    let Some(mut result_decimal) = Decimal::from_u128(value_u128) else {
        return Err(WalletError::ConversionError(format!(
            "could not convert u128 to Decimal: {value:?}"
        )));
    };

    // directly set the decimals
    result_decimal
        .set_scale(decimals)
        .map_err(|e| WalletError::ConversionError(format!("could not set scale to decimals: {e:?}")))?;

    result_decimal.normalize_assign(); // remove trailing zeros

    CryptoAmount::try_from(result_decimal).map_err(|e| {
        WalletError::ConversionError(format!(
            "could not convert decimal {result_decimal:?} to crypto amount: {e:?}"
        ))
    })
}

/// Convert a [`CryptoAmount`] to [`U256`] while taking the decimals into account.
fn convert_crypto_amount_to_u256(amount: CryptoAmount, decimals: u32) -> Result<U256> {
    // normalize to remove trailing zeros
    let value_decimal = amount.inner().normalize();

    let scale = value_decimal.scale();

    // if the Decimal has more decimals than we will store in the U256, we cannot accurately represent this value.
    if scale > decimals {
        return Err(WalletError::ConversionError(format!(
            "cannot represent value of {} in a U256 with {} decimals.",
            value_decimal, decimals
        )));
    }

    if value_decimal.is_sign_negative() {
        return Err(WalletError::ConversionError(format!(
            "cannot represent negative values: {}",
            value_decimal
        )));
    }

    // create a U256 from all the mantissa bits, then we just need to multiply by 10^(decimals - scale) to get the scaled value
    let mantissa = U256::try_from(value_decimal.mantissa())
        .map_err(|e| WalletError::ConversionError(format!("mantissa does not fit in U256: {e}",)))?;

    // the scale is 10^(decimals-scale). Since we checked for scale > decimals above, (decimals - scale) >= 0
    let exponent = U256::from(decimals - scale);
    let scale = U256::from(10)
        .checked_pow(exponent)
        .ok_or_else(|| WalletError::ConversionError(format!("10^{exponent} does not fit in U256")))?;

    println!(
        "decimals: {decimals}, value: {value_decimal}, mantissa: {}, scale: {}",
        mantissa, scale
    );

    let value = mantissa.checked_mul(scale).ok_or_else(|| {
        WalletError::ConversionError(format!("result does not fit in U256: {} * {}", mantissa, scale))
    })?;

    Ok(value)
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(test, mockall::automock)]
impl WalletUser for WalletImplEvm {
    async fn get_address(&self) -> Result<String> {
        Ok(self.provider.default_signer_address().to_string())
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        let mut total = U256::ZERO;
        for addr in self.provider.signer_addresses() {
            let balance = self.provider.get_balance(addr).await?;
            log::info!("Balance for address {} = {}", addr, balance);
            total += balance;
        }

        let balance_eth_crypto_amount = self.convert_alloy_256_to_crypto_amount(total)?;
        Ok(balance_eth_crypto_amount)
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        let tx_request = self.prepare_transaction(intent)?;
        self.submit_transaction_request(tx_request).await
    }

    // The network does not provide information about historical transactions
    // (they can be retrieved manually, but this is a time-consuming process),
    // so the handling of this method is implemented at the SDK level.
    async fn get_wallet_tx_list(&self, _start: usize, _limit: usize) -> Result<WalletTxInfoList> {
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

        let Some(receiver_address) = tx.to() else {
            return Err(WalletError::InvalidTransaction(
                "Transaction has no to address".to_string(),
            ));
        };

        let is_transaction_incoming = receiver_address.to_string() == my_address;

        let receipt = self.provider.get_transaction_receipt(transaction_hash).await?;
        let status = match receipt.map(|r| r.inner.is_success()) {
            Some(true) => InclusionState::Confirmed,
            Some(false) => InclusionState::Conflicting,
            None => InclusionState::Pending,
        };

        let value_eth_crypto_amount = self.convert_alloy_256_to_crypto_amount(tx.value())?;

        let value_eth_f64: f64 = value_eth_crypto_amount.inner().try_into()?; // TODO: WalletTxInfo f64 -> Decimal ? maybe

        Ok(WalletTxInfo {
            date: date.map(|n| n.to_string()).unwrap_or_else(String::new),
            block_id: tx.block_number.map(|n| n.to_string()),
            transaction_id: transaction_id.to_string(),
            receiver: receiver_address.to_string(),
            incoming: is_transaction_incoming,
            amount: value_eth_f64,
            network_key: "ETH".to_string(),
            status: format!("{:?}", status),
            explorer_url: None,
        })
    }

    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation> {
        let tx_request = self.prepare_transaction(intent)?;
        self.estimate_transaction_request_gas(tx_request).await
    }
}

alloy::sol!(
    #[sol(rpc)]
    Erc20Contract,
    "src/abi/erc20.json"
);

/// [`WalletUser`] implementation for EVM-ERC20
#[derive(Debug)]
pub struct WalletImplEvmErc20 {
    inner: WalletImplEvm,
    contract_address: Address,
}
impl WalletImplEvmErc20 {
    /// Creates a new [`WalletImplEvm`] from the specified [`Mnemonic`].
    pub fn new(
        mnemonic: Mnemonic,
        node_urls: Vec<String>,
        chain_id: u64,
        decimals: u32,
        coin_type: u32,
        contract_address: String,
    ) -> Result<Self> {
        Ok(Self {
            inner: WalletImplEvm::new(mnemonic, node_urls, chain_id, decimals, coin_type)?,
            contract_address: contract_address.parse()?,
        })
    }

    fn get_contract(&self) -> Erc20Contract::Erc20ContractInstance<&ProviderType, Ethereum> {
        Erc20Contract::new(self.contract_address, &self.inner.provider)
    }

    /// Helper function that prepares the [`TransactionRequest`] so that we can also use the same logic for gas estimation.
    fn prepare_transaction(&self, intent: &TransactionIntent) -> Result<TransactionRequest> {
        let TransactionIntent {
            address_to,
            amount,
            data,
        } = intent;

        if data.as_ref().is_some_and(|d| !d.is_empty()) {
            // We cannot attach data to the smart contract transfer, so log a warning
            log::warn!("Trying to attach data to an ERC20 Token transfer which will be ignored: {data:?}");
        }

        let addr_to = Address::from_str(address_to)?;
        let amount_wei_u256 = self.inner.convert_crypto_amount_to_u256(*amount)?;

        let contract = self.get_contract();

        // create a TransactionReqeust encoding the contract call
        Ok(contract.transfer(addr_to, amount_wei_u256).into_transaction_request())
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(test, mockall::automock)]
impl WalletUser for WalletImplEvmErc20 {
    async fn get_address(&self) -> Result<String> {
        self.inner.get_address().await
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        let contract = self.get_contract();

        let mut total = U256::ZERO;
        for addr in self.inner.provider.signer_addresses() {
            // call the smart contract here
            let balance = contract.balanceOf(addr).call().await?;

            log::info!("Balance for address {} = {}", addr, balance);
            total += balance;
        }

        let balance_eth_crypto_amount = self.inner.convert_alloy_256_to_crypto_amount(total)?;
        Ok(balance_eth_crypto_amount)
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        let tx_request = self.prepare_transaction(intent)?;
        self.inner.submit_transaction_request(tx_request).await
    }

    // The network does not provide information about historical transactions
    // (they can be retrieved manually, but this is a time-consuming process),
    // so the handling of this method is implemented at the SDK level.
    async fn get_wallet_tx_list(&self, _start: usize, _limit: usize) -> Result<WalletTxInfoList> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn get_wallet_tx(&self, transaction_id: &str) -> Result<WalletTxInfo> {
        // get the information for the underlying transaction
        let mut info = self.inner.get_wallet_tx(transaction_id).await?;

        // for now we will just patch the information with the info from the ERC20 transfer call
        let transaction_hash = TxHash::from_str(transaction_id)?;
        let transaction = self.inner.provider.get_transaction_by_hash(transaction_hash).await?;

        let Some(tx) = transaction else {
            return Err(WalletError::TransactionNotFound);
        };

        let args = Erc20Contract::transferCall::abi_decode(tx.inner.input())?;

        let value_eth_crypto_amount = self.inner.convert_alloy_256_to_crypto_amount(args._value)?;
        info.amount = value_eth_crypto_amount.inner().try_into()?; // TODO: WalletTxInfo f64 -> Decimal ? maybe

        info.receiver = args._to.to_string();

        Ok(info)
    }

    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation> {
        let tx_request = self.prepare_transaction(intent)?;
        self.inner.estimate_transaction_request_gas(tx_request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Config;
    use iota_sdk::crypto::keys::bip39::Mnemonic;
    use rust_decimal_macros::dec;
    use serde_json::json;
    use testing::CleanUp;

    // Account #0: 0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266 (10000 ETH)
    // Private Key: 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
    pub const HARDHAT_MNEMONIC: &str = "test test test test test test test test test test test junk";

    const ETH_DECIMALS: u32 = 18;
    const ETH_COIN_TYPE: u32 = 60;

    #[rstest::rstest]
    #[case(Some(CryptoAmount::try_from(dec!(1)).unwrap()), 3, U256::from(1000))]
    #[case(Some(CryptoAmount::try_from(dec!(1.0)).unwrap()), 3, U256::from(1000))]
    #[case(Some(CryptoAmount::try_from(dec!(1.01)).unwrap()), 3, U256::from(1010))]
    #[case(Some(CryptoAmount::try_from(dec!(1.010)).unwrap()), 3, U256::from(1010))]
    #[case(Some(CryptoAmount::try_from(dec!(1.00)).unwrap()), ETH_DECIMALS, U256::from(1_000_000_000_000_000_000u128))]
    #[case(Some(CryptoAmount::try_from(dec!(0.000_000_000_000_000_001)).unwrap()), ETH_DECIMALS, U256::from(1))]
    #[case(Some(CryptoAmount::try_from(dec!(1.000_000_000_000_000_001)).unwrap()), ETH_DECIMALS, U256::from(1_000_000_000_000_000_001u128))]
    #[case(None, ETH_DECIMALS, U256::MAX)] // Overflow
    #[case(None, 50, U256::from(1))] // Too many decimals, Underflow
    fn test_convert_alloy_256_to_decimal(
        #[case] expected_amount: Option<CryptoAmount>,
        #[case] decimals: u32,
        #[case] value: U256,
    ) {
        let res = convert_alloy_256_to_crypto_amount(value, decimals);
        if let Some(expected) = expected_amount {
            assert_eq!(res.unwrap(), expected);
        } else {
            res.unwrap_err();
        }
    }

    #[rstest::rstest]
    #[case(CryptoAmount::try_from(dec!(1)).unwrap(), 3, Some(U256::from(1000)))]
    #[case(CryptoAmount::try_from(dec!(1.0)).unwrap(), 3, Some(U256::from(1000)))]
    #[case(CryptoAmount::try_from(dec!(1.01)).unwrap(), 3, Some(U256::from(1010)))]
    #[case(CryptoAmount::try_from(dec!(1.010)).unwrap(), 3, Some(U256::from(1010)))]
    #[case(CryptoAmount::try_from(dec!(1.00)).unwrap(), ETH_DECIMALS, Some(U256::from(1_000_000_000_000_000_000u128)))]
    #[case(CryptoAmount::try_from(dec!(0.000_000_000_000_000_001)).unwrap(), ETH_DECIMALS, Some(U256::from(1)))]
    #[case(CryptoAmount::try_from(dec!(1.000_000_000_000_000_001)).unwrap(), ETH_DECIMALS, Some(U256::from(1_000_000_000_000_000_001u128)))]
    #[case(CryptoAmount::try_from(dec!(1_000_000_000_000_000_000)).unwrap(), 60, None)] // Overflow
    #[case(CryptoAmount::try_from(dec!(0.000_000_000_000_000_000_001)).unwrap(), ETH_DECIMALS, None)] // Underflow
    fn test_convert_crypto_amount_to_alloy_256(
        #[case] amount: CryptoAmount,
        #[case] decimals: u32,
        #[case] expected: Option<U256>,
    ) {
        let res = convert_crypto_amount_to_u256(amount, decimals);

        if let Some(expected) = expected {
            assert_eq!(res.unwrap(), expected);
        } else {
            res.unwrap_err();
        }
    }

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user(mnemonic: impl Into<Mnemonic>) -> (WalletImplEvm, CleanUp) {
        let (_, cleanup) = Config::new_test_with_cleanup();
        let node_url = vec![String::from("https://sepolia.mode.network")];
        let chain_id = 31337;

        let wallet = WalletImplEvm::new(mnemonic.into(), node_url, chain_id, ETH_DECIMALS, ETH_COIN_TYPE)
            .expect("should initialize wallet");
        (wallet, cleanup)
    }

    /// helper function to get a [`WalletUser`] instance.
    async fn get_wallet_user_with_mocked_provider(
        mnemonic: impl Into<Mnemonic>,
        node_url: String,
        chain_id: u64,
    ) -> WalletImplEvm {
        WalletImplEvm::new(mnemonic.into(), vec![node_url], chain_id, ETH_DECIMALS, ETH_COIN_TYPE)
            .expect("could not initialize WalletImplEth")
    }

    #[tokio::test]
    async fn test_get_address() {
        //Arrange
        let (wallet_user, _cleanup) = get_wallet_user(HARDHAT_MNEMONIC).await;

        // Act
        let addr_raw = wallet_user.get_address().await.unwrap();

        // Assert
        let parsed = Address::parse_checksummed(addr_raw, None).unwrap();
        let expected = alloy_primitives::address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        assert_eq!(parsed, expected);
    }

    #[tokio::test]
    async fn test_get_balance() {
        //Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let wallet_addr = wallet_user.get_address().await.unwrap().to_lowercase();

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
        let from = wallet_user.get_address().await.unwrap().to_lowercase();
        let to = String::from("0xb0b0000000000000000000000000000000000000");
        let metadata = String::from("test message").into_bytes();

        let intent = TransactionIntent {
            address_to: to.clone(),
            amount: amount_to_send,
            data: Some(metadata),
        };

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
                    "latest"
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

        // Act
        let transaction_id = wallet_user.send_amount(&intent).await;

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
    async fn should_estimate_gas_cost() {
        // Arrange
        let mut server = mockito::Server::new_async().await;
        let url = server.url();
        let node_url = Url::parse(&url).unwrap();
        let chain_id = 31337;

        let wallet_user = get_wallet_user_with_mocked_provider(HARDHAT_MNEMONIC, node_url.to_string(), chain_id).await;

        let to = String::from("0xb0b0000000000000000000000000000000000000");
        let transaction_data = "data";

        let intent = TransactionIntent {
            address_to: to.clone(),
            amount: CryptoAmount::from(1),
            data: Some(transaction_data.to_string().into_bytes()),
        };

        let expected_estimation = GasCostEstimation {
            gas_limit: 24009,
            max_fee_per_gas: 2000000001,
            max_priority_fee_per_gas: 1,
        };

        let from = wallet_user.get_address().await.unwrap().to_lowercase();
        let mocked_rpc_estimate_gas = server
            .mock("POST", "/")
            .match_header("content-type", "application/json")
            .match_body(mockito::Matcher::PartialJson(json!({
                "jsonrpc": "2.0",
                "id": 0,
                "method": "eth_estimateGas",
                "params": [{"from": from,"to": to,"value": "0xde0b6b3a7640000", "input": "0x64617461"},"pending"],
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
        let result = wallet_user.estimate_gas_cost(&intent).await;

        // Assert
        mocked_rpc_estimate_gas.assert();
        mocked_rpc_eth_fee_history.assert();
        let response = result.unwrap();

        assert_eq!(expected_estimation, response)
    }
}
