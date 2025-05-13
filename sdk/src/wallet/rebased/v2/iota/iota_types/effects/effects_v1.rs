use serde::{Deserialize, Serialize};

use crate::wallet::rebased::v2::iota::iota_types::base_types::EpochId;
use crate::wallet::rebased::v2::iota::iota_types::base_types::ObjectID;
use crate::wallet::rebased::v2::iota::iota_types::base_types::SequenceNumber;
use crate::wallet::rebased::v2::iota::iota_types::base_types::VersionDigest;
use crate::wallet::rebased::v2::iota::iota_types::digests::EffectsAuxDataDigest;
use crate::wallet::rebased::v2::iota::iota_types::digests::TransactionDigest;
use crate::wallet::rebased::v2::iota::iota_types::digests::TransactionEventsDigest;
use crate::wallet::rebased::v2::iota::iota_types::execution_status::ExecutionStatus;
use crate::wallet::rebased::v2::iota::iota_types::gas::GasCostSummary;

use super::TransactionEffectsAPI;
use super::object_change::EffectsObjectChange;

/// The response from processing a transaction or a certified transaction
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TransactionEffectsV1 {
    /// The status of the execution
    pub(crate) status: ExecutionStatus,
    /// The epoch when this transaction was executed.
    pub(crate) executed_epoch: EpochId,
    pub(crate) gas_used: GasCostSummary,
    /// The transaction digest
    pub(crate) transaction_digest: TransactionDigest,
    /// The updated gas object reference, as an index into the `changed_objects`
    /// vector. Having a dedicated field for convenient access.
    /// System transaction that don't require gas will leave this as None.
    pub(crate) gas_object_index: Option<u32>,
    /// The digest of the events emitted during execution,
    /// can be None if the transaction does not emit any event.
    pub(crate) events_digest: Option<TransactionEventsDigest>,
    /// The set of transaction digests this transaction depends on.
    pub(crate) dependencies: Vec<TransactionDigest>,

    /// The version number of all the written Move objects by this transaction.
    pub(crate) lamport_version: SequenceNumber,
    /// Objects whose state are changed in the object store.
    pub(crate) changed_objects: Vec<(ObjectID, EffectsObjectChange)>,
    /// Shared objects that are not mutated in this transaction. Unlike owned
    /// objects, read-only shared objects' version are not committed in the
    /// transaction, and in order for a node to catch up and execute it
    /// without consensus sequencing, the version needs to be committed in
    /// the effects.
    pub(crate) unchanged_shared_objects: Vec<(ObjectID, UnchangedSharedKind)>,
    /// Auxiliary data that are not protocol-critical, generated as part of the
    /// effects but are stored separately. Storing it separately allows us
    /// to avoid bloating the effects with data that are not critical.
    /// It also provides more flexibility on the format and type of the data.
    pub(crate) aux_data_digest: Option<EffectsAuxDataDigest>,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum UnchangedSharedKind {
    /// Read-only shared objects from the input. We don't really need
    /// ObjectDigest for protocol correctness, but it will make it easier to
    /// verify untrusted read.
    ReadOnlyRoot(VersionDigest),
    /// Deleted shared objects that appear mutably/owned in the input.
    MutateDeleted(SequenceNumber),
    /// Deleted shared objects that appear as read-only in the input.
    ReadDeleted(SequenceNumber),
    /// Shared objects in cancelled transaction. The sequence number embed
    /// cancellation reason.
    Cancelled(SequenceNumber),
    /// Read of a per-epoch config object that should remain the same during an
    /// epoch.
    PerEpochConfig,
}

impl TransactionEffectsAPI for TransactionEffectsV1 {
    fn gas_cost_summary(&self) -> &GasCostSummary {
        &self.gas_used
    }
}
