// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0

// From https://github.com/iotaledger/iota/blob/develop/crates/iota-json-rpc-api/src/read.rs

use jsonrpsee::proc_macros::rpc;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{
    super::{CheckpointDigest, CheckpointSequenceNumber, EpochId, TransactionDigest, bigint::BigInt},
    IotaTransactionBlockResponse, IotaTransactionBlockResponseOptions,
};

/// Provides methods for reading transaction related data such as transaction
/// blocks, checkpoints, and protocol configuration. The trait further provides
/// methods for reading the ledger (current objects) as well its history (past
/// objects).
#[rpc(client, namespace = "iota")]
pub trait ReadApi {
    /// Return the transaction response object.
    #[rustfmt::skip]
    #[method(name = "getTransactionBlock")]
    async fn get_transaction_block(
        &self,
        // the digest of the queried transaction
        digest: TransactionDigest,
        // options for specifying the content to be returned
        options: Option<IotaTransactionBlockResponseOptions>,
    ) -> RpcResult<IotaTransactionBlockResponse>;

    /// Return a checkpoint
    #[rustfmt::skip]
    #[method(name = "getCheckpoint")]
    async fn get_checkpoint(
        &self,
        // Checkpoint identifier, can use either checkpoint digest, or checkpoint sequence number as input.
        id: CheckpointId,
    ) -> RpcResult<Checkpoint>;
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    /// Checkpoint's epoch ID
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
    /// Checkpoint sequence number
    #[serde_as(as = "BigInt<u64>")]
    pub sequence_number: CheckpointSequenceNumber,
    /// Checkpoint digest
    pub digest: CheckpointDigest,
    // /// Total number of transactions committed since genesis, including those in
    // /// this checkpoint.
    // #[serde_as(as = "BigInt<u64>")]
    // pub network_total_transactions: u64,
    // /// Digest of the previous checkpoint
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub previous_digest: Option<CheckpointDigest>,
    // /// The running total gas costs of all transactions included in the current
    // /// epoch so far until this checkpoint.
    // pub epoch_rolling_gas_cost_summary: GasCostSummary,
    // /// Timestamp of the checkpoint - number of milliseconds from the Unix epoch
    // /// Checkpoint timestamps are monotonic, but not strongly monotonic -
    // /// subsequent checkpoints can have same timestamp if they originate
    // /// from the same underlining consensus commit
    // #[serde_as(as = "BigInt<u64>")]
    // pub timestamp_ms: CheckpointTimestamp,
    // /// Present only on the final checkpoint of the epoch.
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub end_of_epoch_data: Option<EndOfEpochData>,
    // /// Transaction digests
    // pub transactions: Vec<TransactionDigest>,
    //
    // /// Commitments to checkpoint state
    // pub checkpoint_commitments: Vec<CheckpointCommitment>,
    // /// Validator Signature
    // //#[serde_as(as = "Readable<Base64, Bytes>")]
    // pub validator_signature: AggregateAuthoritySignature,
}

#[serde_as]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckpointId {
    SequenceNumber(#[serde_as(as = "BigInt<u64>")] CheckpointSequenceNumber),
    Digest(CheckpointDigest),
}

impl From<CheckpointSequenceNumber> for CheckpointId {
    fn from(seq: CheckpointSequenceNumber) -> Self {
        Self::SequenceNumber(seq)
    }
}

impl From<CheckpointDigest> for CheckpointId {
    fn from(digest: CheckpointDigest) -> Self {
        Self::Digest(digest)
    }
}
