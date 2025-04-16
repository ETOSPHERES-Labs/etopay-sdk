use std::path::Path;

use super::error::{Result, WalletError};
use super::wallet::{TransactionIntent, WalletUser};
use crate::types::{
    currencies::CryptoAmount,
    transactions::{GasCostEstimation, WalletTxInfo, WalletTxInfoList},
};
use async_trait::async_trait;
use iota_keys::keystore::{AccountKeystore, InMemKeystore};
use iota_sdk::crypto::keys::bip39::Mnemonic;
use iota_sdk_rebased::{IotaClient, IotaClientBuilder};

pub struct WalletImplIotaRebased {
    client: IotaClient,
    keystore: InMemKeystore,
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
    pub async fn new(mnemonic: Mnemonic, path: &Path, coin_type: u32, node_url: &[String]) -> Result<Self> {
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

        Ok(Self { client, keystore })
    }
}
#[allow(unused_variables)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl WalletUser for WalletImplIotaRebased {
    async fn get_address(&self) -> Result<String> {
        Ok(self.keystore.addresses[0].to_string())
    }

    async fn get_balance(&self) -> Result<CryptoAmount> {
        todo!()
    }

    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String> {
        todo!()
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
