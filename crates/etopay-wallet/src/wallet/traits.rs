use crate::error::Result;
use crate::types::{CryptoAmount, GasCostEstimation, WalletTxInfo, WalletTxInfoList};
use async_trait::async_trait;
use std::fmt::Debug;

/// The intended transaction to perform. Used to perform the transaction and estimate gas fees.
pub struct TransactionIntent {
    /// The address to send to.
    pub address_to: String,

    /// The amount to send.
    pub amount: CryptoAmount,

    /// Optional data to attach to the transaction.
    pub data: Option<Vec<u8>>,
}

#[cfg_attr(any(test, feature = "mock"), mockall::automock)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
/// Wallet user interface
pub trait WalletUser: Debug {
    /// Gets a new address for the user
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the generated address as a `String` if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, generate addresses, or encounter any other issues.
    async fn get_address(&self) -> Result<String>;

    /// Gets the balance of a user.
    ///
    /// # Returns
    ///
    /// Returns the available balance of the user as a `f64` if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet or encounters any other issues.
    async fn get_balance(&self) -> Result<CryptoAmount>;

    /// Send amount to receiver
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the receiver.
    /// * `amount` - The amount to send.
    /// * `tag` - The transactions tag. Optional.
    /// * `message` - The transactions message. Optional.
    ///
    ///
    /// Returns a `Result` containing the sent transaction ID if successful, or an `Error` if it fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if it fails to synchronize the wallet, send the transaction, or encounter any other issues.
    async fn send_amount(&self, intent: &TransactionIntent) -> Result<String>;

    /// Gets the list of transactions
    ///
    /// # Arguments
    ///
    /// * `start` - The index of the first wallet transaction to return
    /// * `limit` - The number of following wallet transactions to return
    ///
    /// # Returns
    ///
    /// The list of wallet transactions.
    ///
    /// # Errors
    ///
    /// This function can return an error if it cannot retrieve the list of wallet transactions.
    async fn get_wallet_tx_list(&self, start: usize, limit: usize) -> Result<WalletTxInfoList>;

    /// Get detailed report of a particular transaction in the history
    ///
    /// # Arguments
    ///
    /// * `tx_id` - The id of the wallet transaction to return the details for.
    ///
    /// # Returns
    ///
    /// The wallet transaction details.
    ///
    /// # Errors
    ///
    /// This function can return an error if it cannot retrieve the wallet transaction.
    async fn get_wallet_tx(&self, tx_id: &str) -> Result<WalletTxInfo>;

    /// Estimate gas cost for eip 1559 transaction
    ///
    /// # Arguments
    ///
    /// * `transaction` - A transaction with a priority fee ([EIP-1559](https://eips.ethereum.org/EIPS/eip-1559))
    ///
    /// # Returns the estimated gas cost for the underlying transaction to be executed (gas limit, max fee per gas and max priority fee per gas)
    ///
    /// This function can return an error if it cannot parse input transaction or retrieve information from the node.
    async fn estimate_gas_cost(&self, intent: &TransactionIntent) -> Result<GasCostEstimation>;
}
