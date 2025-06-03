use serde::{Deserialize, Serialize};

use crate::types::{WalletTxInfoV1, WalletTxInfoV2};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "version")]
pub enum WalletTxInfoVersioned {
    V1(WalletTxInfoV1),
    V2(WalletTxInfoV2),
}

impl WalletTxInfoVersioned {
    pub fn into_latest(self) -> WalletTxInfoV2 {
        match self {
            WalletTxInfoVersioned::V1(v1) => v1.migrate(),
            WalletTxInfoVersioned::V2(v2) => v2,
        }
    }
}

pub trait Migrate {
    type Next;

    fn migrate(self) -> Self::Next;
}

impl Migrate for WalletTxInfoV1 {
    type Next = WalletTxInfoV2;
    fn migrate(self) -> WalletTxInfoV2 {
        WalletTxInfoV2 {
            date: self.date,
            block_number_hash: self.block_number_hash,
            transaction_hash: self.transaction_hash,
            sender: self.sender,
            receiver: self.receiver,
            amount: self.amount,
            network_key: self.network_key,
            status: self.status,
            explorer_url: self.explorer_url,
            gas: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{CryptoAmount, WalletTxStatus};

    use super::*;
    use chrono::Utc;

    #[test]
    fn test_into_latest_from_v1() {
        // Given
        let date = Utc::now();
        let v1 = WalletTxInfoV1 {
            date,
            transaction_hash: "0x000".to_string(),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(3),
            network_key: "network".to_string(),
            status: WalletTxStatus::Confirmed,
            block_number_hash: None,
            explorer_url: None,
        };

        let expected = WalletTxInfoV2 {
            date,
            transaction_hash: "0x000".to_string(),
            sender: "Satoshi".to_string(),
            receiver: "Bob".to_string(),
            amount: CryptoAmount::from(3),
            network_key: "network".to_string(),
            status: WalletTxStatus::Confirmed,
            block_number_hash: None,
            explorer_url: None,
            gas: None,
        };

        // When
        let versioned = WalletTxInfoVersioned::V1(v1);
        let latest = versioned.into_latest();

        // Then
        assert_eq!(expected, latest);
    }

    // #[test]
    // fn test_into_latest_from_v4_passthrough() {
    //     let date = Utc::now();
    //     let v4 = WalletTxInfoV4 {
    //         date,
    //         transaction_hash: "tx999".to_string(),
    //         sender: "Charlie".to_string(),
    //         receiver: "Dave".to_string(),
    //         amount: CryptoAmount {
    //             value: 500,
    //             unit: "ETH".to_string(),
    //         },
    //         network_key: "eth-mainnet".to_string(),
    //         status: WalletTxStatus::Pending,
    //         block_number_hash: Some((42, "blockhash".to_string())),
    //         explorer_url: Some("https://explorer.etherscan.io/tx/tx999".to_string()),
    //     };

    //     let versioned = WalletTxInfoVersioned::V4(v4.clone());
    //     let latest = versioned.into_latest();

    //     assert_eq!(latest.transaction_hash, v4.transaction_hash);
    //     assert_eq!(latest.amount.value, v4.amount.value);
    //     assert_eq!(latest.date, v4.date);
    // }
}
