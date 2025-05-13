use serde::{Deserialize, Serialize};

use super::{Object, ObjectID, ObjectRef, TransactionDigest};
use crate::wallet::rebased::v2::iota::base_types::SequenceNumber;

/// The result of reading an object for execution. Because shared objects may be
/// deleted, one possible result of reading a shared object is that
/// ObjectReadResultKind::Deleted is returned.
#[derive(Clone, Debug)]
pub struct ObjectReadResult {
    pub input_object_kind: InputObjectKind,
    pub object: ObjectReadResultKind,
}

impl ObjectReadResult {
    pub fn as_object(&self) -> Option<&Object> {
        match &self.object {
            ObjectReadResultKind::Object(object) => Some(object),
            ObjectReadResultKind::DeletedSharedObject(_, _) => None,
            ObjectReadResultKind::CancelledTransactionSharedObject(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord, Hash)]
pub enum InputObjectKind {
    // A Move package, must be immutable.
    MovePackage(ObjectID),
    // A Move object, either immutable, or owned mutable.
    ImmOrOwnedMoveObject(ObjectRef),
    // A Move object that's shared and mutable.
    SharedMoveObject {
        id: ObjectID,
        initial_shared_version: SequenceNumber,
        mutable: bool,
    },
}

#[derive(Clone, Debug)]
pub enum ObjectReadResultKind {
    Object(Object),
    // The version of the object that the transaction intended to read, and the digest of the tx
    // that deleted it.
    DeletedSharedObject(SequenceNumber, TransactionDigest),
    // A shared object in a cancelled transaction. The sequence number embeds cancellation reason.
    CancelledTransactionSharedObject(SequenceNumber),
}
