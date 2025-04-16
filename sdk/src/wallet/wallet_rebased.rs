use super::error::{Result, WalletError};
use super::wallet::{TransactionIntent, WalletUser};
use crate::types::{
    currencies::CryptoAmount,
    transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList},
};
use async_trait::async_trait;
use iota_keys::keystore::{AccountKeystore, InMemKeystore};
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk_rebased::rpc_types::IotaTransactionBlockResponseOptions;
use iota_sdk_rebased::types::base_types::IotaAddress;
use iota_sdk_rebased::types::quorum_driver_types::ExecuteTransactionRequestType;
use iota_sdk_rebased::types::transaction::Transaction;
use iota_sdk_rebased::{IotaClient, IotaClientBuilder};
use iota_shared_crypto::intent::Intent;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

pub struct WalletImplIotaRebased {
    client: IotaClient,
    keystore: InMemKeystore,
    coin_type: String,
    decimals: u32,
}
impl std::fmt::Debug for WalletImplIotaRebased {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WalletImplIotaRebased")
            .field("client", &"<IotaClient>")
            .finish()
    }
}

impl WalletImplIotaRebased {
    /// Creates a new [`WalletImpl`] from the specified [`Config`] and [`Mnemonic`].
    pub async fn new(mnemonic: Mnemonic, coin_type: &str, decimals: u32, node_url: &[String]) -> Result<Self> {
        let client = IotaClientBuilder::default().build(&node_url[0]).await?;
        let mut keystore = InMemKeystore::default();
        keystore
            .import_from_mnemonic(
                &mnemonic,
                iota_sdk_rebased::types::crypto::SignatureScheme::ED25519,
                Some("m/44'/4218'/0'/0'/0'".parse::<bip32::DerivationPath>().unwrap()),
                None,
            )
            .map_err(WalletError::IotaKeys)?;

        Ok(Self {
            client,
            keystore,
            coin_type: coin_type.to_string(),
            decimals,
        })
    }
}

/// Convert a [`U256`] to [`CryptoAmount`] while taking the decimals into account.
fn convert_u128_to_crypto_amount(value: u128, decimals: u32) -> Result<CryptoAmount> {
    let Some(mut result_decimal) = Decimal::from_u128(value) else {
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

#[allow(unused_variables)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletUser for WalletImplIotaRebased {
    async fn get_address(&self) -> Result<String> {
        Ok(self.keystore.addresses()[0].to_string())
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        let address = self.keystore.addresses()[0];
        let balance = self
            .client
            .coin_read_api()
            .get_balance(address, self.coin_type.clone())
            .await?;

        convert_u128_to_crypto_amount(balance.total_balance, self.decimals)
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        let address = self.keystore.addresses()[0];

        let recipient = intent
            .address_to
            .parse::<IotaAddress>()
            .map_err(WalletError::IotaRebasedAnyhow)?;

        let amount = 1000;

        let coins_page = self
            .client
            .coin_read_api()
            .get_coins(address, self.coin_type.clone(), None, None)
            .await?;
        let mut coins = coins_page.data.into_iter();
        let gas_coin = coins.next().expect("missing gas coin");

        let tx_data = self
            .client
            .transaction_builder()
            .pay_iota(
                address,
                vec![gas_coin.coin_object_id], // object to transfer
                vec![recipient],
                vec![amount],
                // gas_coin.coin_object_id, // gas coin
                5_000_000, // gas budget
            )
            .await
            .map_err(WalletError::IotaRebasedAnyhow)?;

        let signature = self
            .keystore
            .sign_secure(&address, &tx_data, Intent::iota_transaction())?;

        let transaction_block_response = self
            .client
            .quorum_driver_api()
            .execute_transaction_block(
                Transaction::from_data(tx_data, vec![signature]),
                IotaTransactionBlockResponseOptions::full_content(),
                ExecuteTransactionRequestType::WaitForLocalExecution,
            )
            .await?;

        log::info!("Transaction sent {}", transaction_block_response.digest);
        log::info!("Object changes:");
        for object_change in transaction_block_response.object_changes.unwrap() {
            log::info!("{:?}", object_change);
        }
        log::info!("Balance changes:");
        for object_change in transaction_block_response.balance_changes.unwrap() {
            log::info!("{:?}", object_change);
        }

        Ok(transaction_block_response.digest.to_string())
    }

    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        todo!()
    }

    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo> {
        todo!()
    }

    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation> {
        todo!()
    }
}
