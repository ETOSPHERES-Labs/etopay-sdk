use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "TransactionBlockEffects", rename_all = "camelCase", tag = "messageVersion")]
pub enum IotaTransactionBlockEffects {
    V1(IotaTransactionBlockEffectsV1),
}

/// The response from processing a transaction or a certified transaction
#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "TransactionBlockEffectsV1", rename_all = "camelCase")]
pub struct IotaTransactionBlockEffectsV1 {
    /// The status of the execution
    pub status: IotaExecutionStatus,
    // /// The epoch when this transaction was executed.
    // #[schemars(with = "BigInt<u64>")]
    // #[serde_as(as = "BigInt<u64>")]
    // pub executed_epoch: EpochId,
    // pub gas_used: GasCostSummary,
    // /// The version that every modified (mutated or deleted) object had before
    // /// it was modified by this transaction.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub modified_at_versions: Vec<IotaTransactionBlockEffectsModifiedAtVersions>,
    // /// The object references of the shared objects used in this transaction.
    // /// Empty if no shared objects were used.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub shared_objects: Vec<IotaObjectRef>,
    // /// The transaction digest
    // pub transaction_digest: TransactionDigest,
    // /// ObjectRef and owner of new objects created.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub created: Vec<OwnedObjectRef>,
    // /// ObjectRef and owner of mutated objects, including gas object.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub mutated: Vec<OwnedObjectRef>,
    // /// ObjectRef and owner of objects that are unwrapped in this transaction.
    // /// Unwrapped objects are objects that were wrapped into other objects in
    // /// the past, and just got extracted out.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub unwrapped: Vec<OwnedObjectRef>,
    // /// Object Refs of objects now deleted (the old refs).
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub deleted: Vec<IotaObjectRef>,
    // /// Object refs of objects previously wrapped in other objects but now
    // /// deleted.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub unwrapped_then_deleted: Vec<IotaObjectRef>,
    // /// Object refs of objects now wrapped in other objects.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub wrapped: Vec<IotaObjectRef>,
    // /// The updated gas object reference. Have a dedicated field for convenient
    // /// access. It's also included in mutated.
    // pub gas_object: OwnedObjectRef,
    // /// The digest of the events emitted during execution,
    // /// can be None if the transaction does not emit any event.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub events_digest: Option<TransactionEventsDigest>,
    // /// The set of transaction digests this transaction depends on.
    // #[serde(default, skip_serializing_if = "Vec::is_empty")]
    // pub dependencies: Vec<TransactionDigest>,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "ExecutionStatus", rename_all = "camelCase", tag = "status")]
pub enum IotaExecutionStatus {
    // Gas used in the success case.
    Success,
    // Gas used in the failed case, and the error.
    Failure { error: String },
}

impl Display for IotaExecutionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Success => write!(f, "success"),
            Self::Failure { error } => write!(f, "failure due to {error}"),
        }
    }
}

impl IotaExecutionStatus {
    pub fn is_ok(&self) -> bool {
        matches!(self, IotaExecutionStatus::Success)
    }
    pub fn is_err(&self) -> bool {
        matches!(self, IotaExecutionStatus::Failure { .. })
    }
}
