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
use iota_sdk_rebased::types::digests::TransactionDigest;
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
            .field("keystore", &"<KeyStore>")
            .field("coin_type", &self.coin_type)
            .field("decimals", &self.decimals)
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

/// Convert a [`u128`] to [`CryptoAmount`] while taking the decimals into account.
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
/// Convert a [`CryptoAmount`] to [`U256`] while taking the decimals into account.
fn convert_crypto_amount_to_u128(amount: CryptoAmount, decimals: u32) -> Result<u128> {
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

    // create a i128 from all the mantissa bits, then we just need to multiply by 10^(decimals - scale) to get the scaled value
    let mantissa = value_decimal.mantissa() as u128; // we checked that it is not negative

    // the scale is 10^(decimals-scale). Since we checked for scale > decimals above, (decimals - scale) >= 0
    let exponent = decimals - scale;
    let scale = 10u128
        .checked_pow(exponent)
        .ok_or_else(|| WalletError::ConversionError(format!("10^{exponent} does not fit in u128")))?;

    let value = mantissa.checked_mul(scale).ok_or_else(|| {
        WalletError::ConversionError(format!("result does not fit in U256: {} * {}", mantissa, scale))
    })?;

    Ok(value)
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

        // TODO: actually check to make sure the u64 can handle the u128 value
        let amount = convert_crypto_amount_to_u128(intent.amount, self.decimals)? as u64;

        let gas_budget = 5_000_000;

        let coins_page = self
            .client
            .coin_read_api()
            .get_coins(address, self.coin_type.clone(), None, None)
            .await?;
        let mut coins = coins_page.data.into_iter();

        // for now we just select _a_ coin object with enough balance, but at some point we probably need
        // to automatically merge multiple objects into one to send them
        let Some(gas_coin) = coins.find(|c| c.balance > (amount + gas_budget)) else {
            return Err(WalletError::InsufficientBalance(String::new()));
        };

        log::info!("using gas_coin: {gas_coin:?}");

        let tx_data = self
            .client
            .transaction_builder()
            .pay_iota(
                address,
                vec![gas_coin.coin_object_id], // object to transfer
                vec![recipient],
                vec![amount],
                // gas_coin.coin_object_id, // gas coin
                gas_budget,
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

        if !transaction_block_response.errors.is_empty() {
            log::warn!("Errors: {:?}", transaction_block_response.errors);
        }

        Ok(transaction_block_response.digest.to_string())
    }

    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo> {
        let digest = tx_id
            .parse::<TransactionDigest>()
            .map_err(WalletError::IotaRebasedAnyhow)?;
        let tx = self
            .client
            .read_api()
            // .get_transaction_with_options(digest, IotaTransactionBlockResponseOptions::with_balance_changes())
            .get_transaction_with_options(digest, IotaTransactionBlockResponseOptions::full_content())
            .await?;

        log::info!("Tx: {tx:#?}");

        // TODO: get the information from the tx, most likely from the balance_changes

        // if let Some(changes) = tx.balance_changes {
        //     if changes.len() == 1 {
        //         // TX to self, only single balance change
        //     }
        //
        //     // for change in tx {
        //     //     change.owner
        //     // }
        // }

        Ok(WalletTxInfo {
            date: String::new(),
            block_id: None,
            transaction_id: tx_id.to_string(),
            incoming: false,
            receiver: String::new(),
            amount: 0.0,
            network_key: String::new(),
            status: String::new(),
            explorer_url: None,
        })
    }

    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation> {
        todo!()
    }
}
