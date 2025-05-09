use serde::{Deserialize, Serialize};

use super::{ObjectID, SequenceNumber, TransactionDigest};

/// Uses an enum to allow for future expansion of the
/// ConsensusDeterminedVersionAssignments.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum ConsensusDeterminedVersionAssignments {
    // Cancelled transaction version assignment.
    CancelledTransactions(Vec<(TransactionDigest, Vec<(ObjectID, SequenceNumber)>)>),
}
