use super::{Digest, Event, ObjectDigest, ObjectID, default_hash, sequence_number::SequenceNumber};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize, Default)]
pub struct TransactionEvents {
    pub data: Vec<Event>,
}

impl TransactionEvents {
    pub fn digest(&self) -> Digest {
        Digest::new(default_hash(self))
    }
}

#[derive(Clone)]
pub struct ObjectChange {
    pub id: ObjectID,
    pub input_version: Option<SequenceNumber>,
    pub input_digest: Option<ObjectDigest>,
    pub output_version: Option<SequenceNumber>,
    pub output_digest: Option<ObjectDigest>,
    pub id_operation: IDOperation,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum IDOperation {
    None,
    Created,
    Deleted,
}
