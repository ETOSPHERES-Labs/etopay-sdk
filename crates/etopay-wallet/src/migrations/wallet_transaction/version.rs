use crate::types::WalletTxInfoV2;

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(tag = "version")]
// pub enum WalletTx {
//     V1(WalletTxInfoV1),
//     V2(WalletTxInfoV2),
// }

// current (latest) version always
pub type WalletTx = WalletTxInfoV2;
