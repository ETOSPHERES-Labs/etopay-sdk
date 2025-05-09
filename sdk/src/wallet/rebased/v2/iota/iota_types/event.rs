use super::BigInt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{Bytes, serde_as};

use super::Readable;
use super::TransactionDigest;

/// Unique ID of an IOTA Event, the ID is a combination of tx seq number and
/// event seq number, the ID is local to this particular fullnode and will be
/// different from other fullnode.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventID {
    pub tx_digest: TransactionDigest,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub event_seq: u64,
}
