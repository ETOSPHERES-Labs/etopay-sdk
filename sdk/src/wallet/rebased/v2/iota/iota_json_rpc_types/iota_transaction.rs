use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};

use super::{IotaEvent, ObjectChange, balance_changes::BalanceChange};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DryRunTransactionBlockResponse {
    pub effects: IotaTransactionBlockEffects,
    pub events: IotaTransactionBlockEvents,
    pub object_changes: Vec<ObjectChange>,
    pub balance_changes: Vec<BalanceChange>,
    pub input: IotaTransactionBlockData,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[enum_dispatch(IotaTransactionBlockEffectsAPI)]
#[serde(rename = "TransactionBlockEffects", rename_all = "camelCase", tag = "messageVersion")]
pub enum IotaTransactionBlockEffects {
    V1(IotaTransactionBlockEffectsV1),
}

#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename = "TransactionBlockEvents", transparent)]
pub struct IotaTransactionBlockEvents {
    pub data: Vec<IotaEvent>,
}
