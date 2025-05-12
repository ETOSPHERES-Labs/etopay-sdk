use crate::wallet::rebased::v2::iota::iota_serde::BigInt;
use crate::wallet::rebased::v2::iota::iota_types::EpochId;
use enum_dispatch::enum_dispatch;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{IotaEvent, ObjectChange, balance_changes::BalanceChange};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DryRunTransactionBlockResponse {
    pub effects: IotaTransactionBlockEffects,
    pub events: IotaTransactionBlockEvents,
    pub object_changes: Vec<ObjectChange>,
    pub balance_changes: Vec<BalanceChange>,
    pub input: IotaTransactionBlockData,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[enum_dispatch(IotaTransactionBlockEffectsAPI)]
#[serde(rename = "TransactionBlockEffects", rename_all = "camelCase", tag = "messageVersion")]
pub enum IotaTransactionBlockEffects {
    V1(IotaTransactionBlockEffectsV1),
}

/// The response from processing a transaction or a certified transaction
#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionBlockEffectsV1", rename_all = "camelCase")]
pub struct IotaTransactionBlockEffectsV1 {
    /// The status of the execution
    pub status: IotaExecutionStatus,
    /// The epoch when this transaction was executed.
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub executed_epoch: EpochId,
    pub gas_used: GasCostSummary,
    /// The version that every modified (mutated or deleted) object had before
    /// it was modified by this transaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub modified_at_versions: Vec<IotaTransactionBlockEffectsModifiedAtVersions>,
    /// The object references of the shared objects used in this transaction.
    /// Empty if no shared objects were used.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_objects: Vec<IotaObjectRef>,
    /// The transaction digest
    pub transaction_digest: TransactionDigest,
    /// ObjectRef and owner of new objects created.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of mutated objects, including gas object.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mutated: Vec<OwnedObjectRef>,
    /// ObjectRef and owner of objects that are unwrapped in this transaction.
    /// Unwrapped objects are objects that were wrapped into other objects in
    /// the past, and just got extracted out.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped: Vec<OwnedObjectRef>,
    /// Object Refs of objects now deleted (the old refs).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub deleted: Vec<IotaObjectRef>,
    /// Object refs of objects previously wrapped in other objects but now
    /// deleted.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unwrapped_then_deleted: Vec<IotaObjectRef>,
    /// Object refs of objects now wrapped in other objects.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub wrapped: Vec<IotaObjectRef>,
    /// The updated gas object reference. Have a dedicated field for convenient
    /// access. It's also included in mutated.
    pub gas_object: OwnedObjectRef,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<TransactionDigest>,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "ExecutionStatus", rename_all = "camelCase", tag = "status")]
pub enum IotaExecutionStatus {
    // Gas used in the success case.
    Success,
    // Gas used in the failed case, and the error.
    Failure { error: String },
}

#[derive(Eq, PartialEq, Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename = "TransactionBlockEvents", transparent)]
pub struct IotaTransactionBlockEvents {
    pub data: Vec<IotaEvent>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[enum_dispatch(IotaTransactionBlockDataAPI)]
#[serde(rename = "TransactionBlockData", rename_all = "camelCase", tag = "messageVersion")]
pub enum IotaTransactionBlockData {
    V1(IotaTransactionBlockDataV1),
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Eq)]
#[serde(rename = "TransactionBlockDataV1", rename_all = "camelCase")]
pub struct IotaTransactionBlockDataV1 {
    pub transaction: IotaTransactionBlockKind,
    pub sender: IotaAddress,
    pub gas_data: IotaGasData,
}
