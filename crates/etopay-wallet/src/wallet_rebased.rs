use std::ops::{Add, Sub};
use std::time::{Duration, Instant};

use super::error::{Result, WalletError};
use super::rebased::{
    self, Argument, CoinReadApiClient, Command, GasData, IotaAddress, ObjectArg, ProgrammableTransactionBuilder,
    RebasedError, RpcClient, TransactionData, TransactionExpiration,
};
use super::wallet::{TransactionIntent, WalletUser};
use crate::rebased::{
    GovernanceReadApiClient, IotaTransactionBlockEffects, Owner, ReadApiClient, TransactionKind, WriteApiClient,
};
use crate::types::{CryptoAmount, GasCostEstimation, InclusionState, WalletTxInfo, WalletTxInfoList};
use async_trait::async_trait;
use bip39::Mnemonic;
use chrono::{TimeZone, Utc};
use jsonrpsee::types::ErrorCode;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

const WAIT_FOR_LOCAL_EXECUTION_TIMEOUT: Duration = Duration::from_secs(60);
const WAIT_FOR_LOCAL_EXECUTION_DELAY: Duration = Duration::from_millis(200);
const WAIT_FOR_LOCAL_EXECUTION_INTERVAL: Duration = Duration::from_secs(2);

pub struct WalletImplIotaRebased {
    client: super::rebased::RpcClient,
    keystore: rebased::InMemKeystore,
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
        let mut keystore2 = rebased::InMemKeystore::default();
        keystore2.import_from_mnemonic(mnemonic, "m/44'/4218'/0'/0'/0'".parse::<bip32::DerivationPath>()?)?;

        let client = RpcClient::new(&node_url[0]).await?;

        Ok(Self {
            client,
            keystore: keystore2,
            coin_type: coin_type.to_string(),
            decimals,
        })
    }
}

/// Convert a [`u128`] to [`CryptoAmount`] while taking the decimals into account.
#[allow(clippy::result_large_err)]
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
#[allow(clippy::result_large_err)]
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
            .get_balance(address, Some(self.coin_type.clone()))
            .await
            .map_err(RebasedError::RpcError)?;

        convert_u128_to_crypto_amount(balance.total_balance, self.decimals)
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        let address = self.keystore.addresses()[0];

        let recipient = intent.address_to.parse::<rebased::IotaAddress>()?;

        // TODO: actually check to make sure the u64 can handle the u128 value
        let amount = convert_crypto_amount_to_u128(intent.amount, self.decimals)? as u64;

        let tx_data = self.prepare_tx_data(recipient, amount).await?;

        let signature = self
            .keystore
            .sign_secure(&address, &tx_data, rebased::Intent::iota_transaction())?;

        let tx = rebased::Transaction::from_data(tx_data, vec![signature.clone()]);

        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures()?;

        let start = Instant::now();
        let transaction_block_response = self
            .client
            .execute_transaction_block(
                tx_bytes.clone(),
                signatures.clone(),
                Some(rebased::IotaTransactionBlockResponseOptions::default()),
                None,
            )
            .await
            .map_err(RebasedError::RpcError)?;

        log::info!("Transaction submitted {}", transaction_block_response.digest);

        // JSON-RPC ignores WaitForLocalExecution, so simulate it by polling for the
        // transaction.
        let poll_response = tokio::time::timeout(WAIT_FOR_LOCAL_EXECUTION_TIMEOUT, async {
            // Apply a short delay to give the full node a chance to catch up.
            tokio::time::sleep(WAIT_FOR_LOCAL_EXECUTION_DELAY).await;

            let mut interval = tokio::time::interval(WAIT_FOR_LOCAL_EXECUTION_INTERVAL);
            loop {
                interval.tick().await;

                if let Ok(poll_response) = self
                    .client
                    .get_transaction_block(transaction_block_response.digest, None)
                    .await
                {
                    break poll_response;
                }
            }
        })
        .await
        .map_err(|_| {
            WalletError::FailToConfirmTransactionStatus(
                transaction_block_response.digest.to_string(),
                start.elapsed().as_secs(),
            )
        })?;

        log::info!("Response:\n{poll_response:?}");

        if !transaction_block_response.errors.is_empty() {
            log::warn!("Errors: {:?}", transaction_block_response.errors);
        }

        Ok(poll_response.digest.to_string())
    }

    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList> {
        Err(WalletError::WalletFeatureNotImplemented)
    }

    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo> {
        let digest = tx_id.parse::<rebased::TransactionDigest>()?;

        let tx = self
            .client
            .get_transaction_block(
                digest,
                Some(rebased::IotaTransactionBlockResponseOptions::full_content()),
            )
            .await
            .map_err(|e| match &e {
                jsonrpsee::core::client::Error::Call(r) if r.code() == ErrorCode::InvalidParams.code() => {
                    WalletError::TransactionNotFound
                }
                _ => WalletError::IotaRebased(RebasedError::RpcError(e)),
            })?;

        log::info!("Transaction Details:\n{tx:?}");

        // The timestamp is in milliseconds but we make it into a human-readable format
        let date = tx
            .timestamp_ms
            .and_then(|n| Utc.timestamp_millis_opt(n as i64).single())
            .map(|dt| dt.to_rfc3339())
            .unwrap_or_default(); // default is going to be an empty String here

        // For block id we use the checkpoint number which shows when the tx was finalized.
        let block_id = tx.checkpoint.map(|n| n.to_string());

        let status = match tx.effects.map(|effects| match effects {
            IotaTransactionBlockEffects::V1(inner) => inner.status.is_ok(),
        }) {
            Some(true) => InclusionState::Confirmed,
            Some(false) => InclusionState::Conflicting,
            None => InclusionState::Pending,
        };

        // 1) Pull out raw u128s for amount and fee, plus sender / receiver addresses
        let (sender, receiver, raw_amount, raw_fee) = match tx.balance_changes.as_ref() {
            Some(changes) => {
                // a) Find the negative change (spent = amount + fee)
                if let Some(neg) = changes.iter().find(|bc| bc.amount < 0) {
                    let sender = Some(neg.owner);
                    // convert to positive u128
                    let spent = (-neg.amount) as u128;

                    // b) See if there’s a positive change (external send)
                    if let Some(pos) = changes.iter().find(|bc| bc.amount > 0) {
                        let receiver = Some(pos.owner);
                        let amount = pos.amount as u128;
                        let fee = spent.saturating_sub(amount);
                        (sender, receiver, amount, fee)
                    } else {
                        // no positive entry → self-send
                        // amount = 0, fee = everything they “spent”
                        // sender and receiver is the same
                        (sender, sender, 0, spent)
                    }
                } else {
                    // no negative entry → malformed or zero-change tx
                    (None, None, 0, 0)
                }
            }
            None => {
                // no balance_changes at all
                (None, None, 0, 0)
            }
        };

        // 2) Turn amount into f64
        let amount = convert_u128_to_crypto_amount(raw_amount, self.decimals)?;

        let receiver = receiver
            .map(|owner| match owner {
                Owner::AddressOwner(addr) | Owner::ObjectOwner(addr) => Ok(addr.to_string()),
                _ => Err(WalletError::WalletFeatureNotImplemented),
            })
            .unwrap_or(Ok(String::default()))?;

        let sender = sender
            .map(|owner| match owner {
                Owner::AddressOwner(addr) | Owner::ObjectOwner(addr) => Ok(addr.to_string()),
                _ => Err(WalletError::WalletFeatureNotImplemented),
            })
            .unwrap_or(Ok(String::default()))?;

        Ok(WalletTxInfo {
            date,
            block_id,
            transaction_id: tx_id.to_string(),
            sender,
            receiver,
            amount,
            network_key: String::from("iota_rebased_testnet"),
            status,
            explorer_url: None,
        })
    }

    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation> {
        let address = self.keystore.addresses()[0];
        let recipient = intent.address_to.parse::<rebased::IotaAddress>()?;
        let amount = convert_crypto_amount_to_u128(intent.amount, self.decimals)? as u64;

        let tx_data = self.prepare_tx_data(recipient, amount).await?;

        let signature = self
            .keystore
            .sign_secure(&address, &tx_data, rebased::Intent::iota_transaction())?;

        let tx = rebased::Transaction::from_data(tx_data, vec![signature.clone()]);

        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures()?;

        let dry_run_tx_block_resp = self
            .client
            .dry_run_transaction_block(tx_bytes.clone())
            .await
            .map_err(RebasedError::RpcError)?;

        let gas_used = self.get_total_gas_used(dry_run_tx_block_resp.effects);

        log::info!("Estimate gas: gas used: {gas_used:?}");

        Ok(GasCostEstimation {
            max_fee_per_gas: 0,
            max_priority_fee_per_gas: 0,
            gas_limit: gas_used,
        })
    }
}

impl WalletImplIotaRebased {
    fn get_total_gas_used(&self, transaction_block_effects: IotaTransactionBlockEffects) -> u64 {
        match transaction_block_effects {
            IotaTransactionBlockEffects::V1(iota_transaction_block_effects_v1) => {
                let gas_summary = iota_transaction_block_effects_v1.gas_used;

                gas_summary
                    .computation_cost
                    .add(gas_summary.storage_cost)
                    .sub(gas_summary.storage_rebate)
            }
        }
    }

    async fn prepare_tx_data(
        &self,
        recipient: IotaAddress,
        amount: u64,
    ) -> core::result::Result<TransactionData, RebasedError> {
        let address = self.keystore.addresses()[0];

        let gas_budget = 5_000_000;

        let mut coins = self
            .client
            .get_coins(address, Some(self.coin_type.clone()), None, None)
            .await
            .map_err(RebasedError::RpcError)?
            .data;

        // for now we just select _a_ coin object with enough balance, but at some point we probably need
        // to automatically merge multiple objects into one to send them

        let (mut builder, gas_coin) = if let Some(gas_coin) = coins.iter().find(|c| c.balance > (amount + gas_budget)) {
            log::info!("Single coin to cover gas and transaction found: {gas_coin:?}");

            (ProgrammableTransactionBuilder::new(), gas_coin.clone())
        } else {
            // we do not have a single coin to cover amount + gas budget. Try to merge multiple
            // coins until we have enough.

            // first find a coin to cover the gas budget (probably must be iota coin)
            let Some(gas_coin_idx) = coins.iter().position(|c| c.balance >= gas_budget) else {
                // not found -> no way to cover the costs!
                return Err(RebasedError::InsufficientBalance(String::new()));
            };

            // take out the gas coin
            let gas_coin = coins.swap_remove(gas_coin_idx);

            let mut total = gas_coin.balance;

            let mut other_coins = Vec::new();

            for coin in coins.into_iter() {
                // if we have enough, stop here
                if total >= (amount + gas_budget) {
                    break;
                }
                // otherwise add this coin to the list
                total += coin.balance;
                other_coins.push(coin);
            }

            // if we didn't find enough funds, error!
            if total < (amount + gas_budget) {
                return Err(RebasedError::InsufficientBalance(format!(
                    "Required: {}, found: {}",
                    amount + gas_budget,
                    total
                )));
            }

            // we now have:
            // - gas_coin that can cover the gas costs
            // - a list of other coins that, when merged with the gas_coin, covers the total amount

            log::info!("Gas Coin: {gas_coin:?}");
            log::info!("Merging {} other Coins", other_coins.len());
            log::info!("Total balance: {total}");

            let mut b = ProgrammableTransactionBuilder::new();

            // put all other coins into the arguments
            let input_other_coins = other_coins
                .iter()
                .map(|c| {
                    b.obj(ObjectArg::ImmOrOwnedObject(c.obj_ref()))
                        .map_err(RebasedError::BuilderError)
                })
                .collect::<core::result::Result<Vec<_>, rebased::RebasedError>>()?;

            if !input_other_coins.is_empty() {
                // Merge the other coins into the GasCoin
                b.command(Command::MergeCoins(Argument::GasCoin, input_other_coins));
            }

            (b, gas_coin)
        };

        // At this point we have a ProgrammableTransactionBuilder that has inputs and commands (if
        // needed) to have enough Balance in the GasCoin to cover the transaction.
        // So we just append the logic to perform the split and transfer:

        // provide the inputs
        let input_amount = builder.pure(amount).map_err(RebasedError::BuilderError)?;
        let input_receiver = builder.pure(recipient).map_err(RebasedError::BuilderError)?;

        // split the gas coin depending on the amount to send
        let Argument::Result(split_primary) =
            builder.command(Command::SplitCoins(Argument::GasCoin, vec![input_amount]))
        else {
            panic!("self.command should always give a Argument::Result")
        };

        // actually transfer the object that resulted from the split
        builder.command(Command::TransferObjects(
            vec![Argument::NestedResult(split_primary, 0)],
            input_receiver,
        ));

        let pt = builder.finish();

        let gas_price = self
            .client
            .get_reference_gas_price()
            .await
            .map_err(RebasedError::RpcError)?;

        let tx_data = rebased::TransactionData::V1(rebased::TransactionDataV1 {
            kind: TransactionKind::ProgrammableTransaction(pt),
            sender: address,
            gas_data: GasData {
                payment: vec![gas_coin.obj_ref()],
                owner: address,
                price: *gas_price,
                budget: gas_budget,
            },
            expiration: TransactionExpiration::None,
        });

        Ok(tx_data)
    }
}
