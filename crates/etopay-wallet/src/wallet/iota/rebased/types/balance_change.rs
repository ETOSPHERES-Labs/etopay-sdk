use super::Owner;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BalanceChange {
    /// Owner of the balance change
    pub owner: Owner,
    // #[serde_as(as = "IotaTypeTag")]
    // pub coin_type: TypeTag,
    pub coin_type: serde_json::Value,
    /// The amount indicate the balance value changes,
    /// negative amount means spending coin value and positive means receiving
    /// coin value.
    #[serde_as(as = "DisplayFromStr")]
    pub amount: i128,
}
