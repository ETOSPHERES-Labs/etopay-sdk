use serde::{Deserialize, Serialize};

use crate::types::{WalletTxInfoV1, WalletTxInfoV2};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "version")]
pub enum WalletTx {
    V1(WalletTxInfoV1),
    V2(WalletTxInfoV2),
}

pub type WalletTxLatest = WalletTxInfoV2;
