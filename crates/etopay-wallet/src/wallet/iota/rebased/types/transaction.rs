// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// From https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/transaction.rs

use serde::{Deserialize, Serialize};

use super::super::RebasedError;
use super::super::encoding::Base64;
use super::super::{Intent, IntentMessage};
use super::{
    Envelope, GenericSignature, IntentScope, IotaAddress, Message, ObjectRef, ProgrammableTransaction, Signature,
    SizeOneVec, TransactionDigest, default_hash,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EmptySignInfo {}

pub type Transaction = Envelope<SenderSignedData, EmptySignInfo>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SenderSignedData(SizeOneVec<SenderSignedTransaction>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SenderSignedTransaction {
    pub intent_message: IntentMessage<TransactionData>,
    /// A list of signatures signed by all transaction participants.
    /// 1. non participant signature must not be present.
    /// 2. signature order does not matter.
    pub tx_signatures: Vec<GenericSignature>,
}

impl Serialize for SenderSignedTransaction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(rename = "SenderSignedTransaction")]
        struct SignedTxn<'a> {
            intent_message: &'a IntentMessage<TransactionData>,
            tx_signatures: &'a Vec<GenericSignature>,
        }

        if self.intent_message().intent != Intent::iota_transaction() {
            return Err(serde::ser::Error::custom("invalid Intent for Transaction"));
        }

        let txn = SignedTxn {
            intent_message: self.intent_message(),
            tx_signatures: &self.tx_signatures,
        };
        txn.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SenderSignedTransaction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename = "SenderSignedTransaction")]
        struct SignedTxn {
            intent_message: IntentMessage<TransactionData>,
            tx_signatures: Vec<GenericSignature>,
        }

        let SignedTxn {
            intent_message,
            tx_signatures,
        } = Deserialize::deserialize(deserializer)?;

        if intent_message.intent != Intent::iota_transaction() {
            return Err(serde::de::Error::custom("invalid Intent for Transaction"));
        }

        Ok(Self {
            intent_message,
            tx_signatures,
        })
    }
}

impl SenderSignedTransaction {
    pub fn intent_message(&self) -> &IntentMessage<TransactionData> {
        &self.intent_message
    }
}

impl SenderSignedData {
    pub fn new(tx_data: TransactionData, tx_signatures: Vec<GenericSignature>) -> Self {
        Self(SizeOneVec::new(SenderSignedTransaction {
            intent_message: IntentMessage::new(Intent::iota_transaction(), tx_data),
            tx_signatures,
        }))
    }

    pub fn new_from_sender_signature(tx_data: TransactionData, tx_signature: Signature) -> Self {
        Self(SizeOneVec::new(SenderSignedTransaction {
            intent_message: IntentMessage::new(Intent::iota_transaction(), tx_data),
            tx_signatures: vec![tx_signature.into()],
        }))
    }

    pub fn inner(&self) -> &SenderSignedTransaction {
        self.0.element()
    }

    pub fn intent_message(&self) -> &IntentMessage<TransactionData> {
        self.inner().intent_message()
    }
}

// impl<S> Envelope<SenderSignedData, S> {
//     pub fn sender_address(&self) -> IotaAddress {
//         self.data().intent_message().value.sender()
//     }
//
//     pub fn gas(&self) -> &[ObjectRef] {
//         self.data().intent_message().value.gas()
//     }
//
//     pub fn contains_shared_object(&self) -> bool {
//         self.shared_input_objects().next().is_some()
//     }
//
//     pub fn shared_input_objects(&self) -> impl Iterator<Item = SharedInputObject> + '_ {
//         self.data()
//             .inner()
//             .intent_message
//             .value
//             .shared_input_objects()
//             .into_iter()
//     }
//
//     // Returns the primary key for this transaction.
//     pub fn key(&self) -> TransactionKey {
//         match &self.data().intent_message().value.kind() {
//             TransactionKind::RandomnessStateUpdate(rsu) => {
//                 TransactionKey::RandomnessRound(rsu.epoch, rsu.randomness_round)
//             }
//             _ => TransactionKey::Digest(*self.digest()),
//         }
//     }
//
//     // Returns non-Digest keys that could be used to refer to this transaction.
//     //
//     // At the moment this returns a single Option for efficiency, but if more key
//     // types are added, the return type could change to Vec<TransactionKey>.
//     pub fn non_digest_key(&self) -> Option<TransactionKey> {
//         match &self.data().intent_message().value.kind() {
//             TransactionKind::RandomnessStateUpdate(rsu) => {
//                 Some(TransactionKey::RandomnessRound(rsu.epoch, rsu.randomness_round))
//             }
//             _ => None,
//         }
//     }
//
//     pub fn is_system_tx(&self) -> bool {
//         self.data().intent_message().value.is_system_tx()
//     }
//
//     pub fn is_sponsored_tx(&self) -> bool {
//         self.data().intent_message().value.is_sponsored_tx()
//     }
// }
impl Message for SenderSignedData {
    type DigestType = TransactionDigest;
    const SCOPE: IntentScope = IntentScope::SenderSignedTransaction;

    /// Computes the tx digest that encodes the Rust type prefix from Signable
    /// trait.
    fn digest(&self) -> Self::DigestType {
        self.intent_message().value.digest()
    }
}

impl Transaction {
    // TODO: Rename this function and above to make it clearer.
    pub fn from_data(data: TransactionData, signatures: Vec<Signature>) -> Self {
        Self::from_generic_sig_data(data, signatures.into_iter().map(|s| s.into()).collect())
    }

    pub fn from_generic_sig_data(data: TransactionData, signatures: Vec<GenericSignature>) -> Self {
        Self::new(SenderSignedData::new(data, signatures))
    }

    /// Returns the Base64 encoded tx_bytes
    /// and a list of Base64 encoded [enum GenericSignature].
    pub fn to_tx_bytes_and_signatures(&self) -> Result<(Base64, Vec<Base64>), RebasedError> {
        Ok((
            Base64::from_bytes(&bcs::to_bytes(&self.data().intent_message().value)?),
            self.data()
                .inner()
                .tx_signatures
                .iter()
                .map(|s| Base64::from_bytes(s.as_ref()))
                .collect(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct GasData {
    pub payment: Vec<ObjectRef>,
    pub owner: IotaAddress,
    pub price: u64,
    pub budget: u64,
}

pub type EpochId = u64;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)]
pub enum TransactionExpiration {
    /// The transaction has no expiration
    None,
    /// Validators wont sign a transaction unless the expiration Epoch
    /// is greater than or equal to the current epoch
    Epoch(EpochId),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum TransactionData {
    V1(TransactionDataV1),
    // When new variants are introduced, it is important that we check version support
    // in the validity_check function based on the protocol config.
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub struct TransactionDataV1 {
    pub kind: TransactionKind,
    pub sender: IotaAddress,
    pub gas_data: GasData,
    pub expiration: TransactionExpiration,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
pub enum TransactionKind {
    /// A transaction that allows the interleaving of native commands and Move
    /// calls
    ProgrammableTransaction(ProgrammableTransaction),
    // /// A system transaction that will update epoch information on-chain.
    // /// It will only ever be executed once in an epoch.
    // /// The argument is the next epoch number, which is critical
    // /// because it ensures that this transaction has a unique digest.
    // /// This will eventually be translated to a Move call during execution.
    // /// It also doesn't require/use a gas object.
    // /// A validator will not sign a transaction of this kind from outside. It
    // /// only signs internally during epoch changes.
    // Genesis(GenesisTransaction),
    // ConsensusCommitPrologueV1(ConsensusCommitPrologueV1),
    // AuthenticatorStateUpdateV1(AuthenticatorStateUpdateV1),
    //
    // /// EndOfEpochTransaction contains a list of transactions
    // /// that are allowed to run at the end of the epoch.
    // EndOfEpochTransaction(Vec<EndOfEpochTransactionKind>),
    //
    // RandomnessStateUpdate(RandomnessStateUpdate),
    // .. more transaction types go here
}

impl TransactionData {
    pub fn digest(&self) -> TransactionDigest {
        TransactionDigest::new(default_hash(self))
    }
}
