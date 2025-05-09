// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2024 IOTA Stiftung
// Modifications Copyright (c) 2025 ETO GRUPPE TECHNOLOGIES GmbH
// SPDX-License-Identifier: Apache-2.0
//
// From https://github.com/iotaledger/iota/blob/develop/crates/iota-types/src/transaction.rs

use std::fmt::{self, Display, Formatter};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::wallet::rebased::RebasedError;
//use crate::wallet::rebased::iota_json::IotaJsonValue;
use crate::wallet::rebased::serde::SequenceNumber as AsSequenceNumber;

use super::super::encoding::Base64;
use super::super::{Intent, IntentMessage};
// use super::{
//     ActiveJwk, CheckpointDigest, ConsensusCommitDigest, ConsensusDeterminedVersionAssignments, Envelope, EventID,
//     GenericSignature, IntentScope, IotaAddress, IotaObjectRef, IotaTypeTag as AsIotaTypeTag, Message, ObjectDigest,
//     ObjectID, ObjectRef, ProgrammableTransaction, SequenceNumber, Signature, SizeOneVec, TransactionDigest, TypeTag,
//     default_hash,
// };

use super::{
    CheckpointDigest, ConsensusCommitDigest, ConsensusDeterminedVersionAssignments, Envelope, EventID,
    GenericSignature, IntentScope, IotaAddress, IotaObjectRef, Message, ObjectDigest, ObjectID, ObjectRef,
    ProgrammableTransaction, SequenceNumber, Signature, SizeOneVec, TransactionDigest, TypeTag, default_hash,
};
//let x:IotaTypeTag;
//IotaTypeTag as AsIotaTypeTag,
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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
//#[enum_dispatch(IotaTransactionBlockDataAPI)]
#[serde(rename = "TransactionBlockData", rename_all = "camelCase", tag = "messageVersion")]
pub enum IotaTransactionBlockData {
    V1(IotaTransactionBlockDataV1),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "TransactionBlockDataV1", rename_all = "camelCase")]
pub struct IotaTransactionBlockDataV1 {
    pub transaction: IotaTransactionBlockKind,
    pub sender: IotaAddress,
    pub gas_data: IotaGasData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "TransactionBlockKind", tag = "kind")]
pub enum IotaTransactionBlockKind {
    /// A system transaction used for initializing the initial state of the
    /// chain.
    //Genesis(IotaGenesisTransaction),
    /// A system transaction marking the start of a series of transactions
    /// scheduled as part of a checkpoint
    ConsensusCommitPrologueV1(IotaConsensusCommitPrologueV1),
    // A series of transactions where the results of one transaction can be
    // used in future transactions
    // -> ProgrammableTransaction(IotaProgrammableTransactionBlock),
    // A transaction which updates global authenticator state
    //AuthenticatorStateUpdateV1(IotaAuthenticatorStateUpdateV1),
    // A transaction which updates global randomness state
    //RandomnessStateUpdate(IotaRandomnessStateUpdate),
    // The transaction which occurs only at the end of the epoch
    //EndOfEpochTransaction(IotaEndOfEpochTransaction),
    // .. more transaction types go here
}

use crate::wallet::rebased::bigint::BigInt;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename = "GasData", rename_all = "camelCase")]
pub struct IotaGasData {
    pub payment: Vec<IotaObjectRef>,
    pub owner: IotaAddress,
    #[serde_as(as = "BigInt<u64>")]
    pub price: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub budget: u64,
}

fn objref_string(obj: &IotaObjectRef) -> String {
    format!(
        " ┌──\n │ ID: {} \n │ Version: {} \n │ Digest: {}\n └──",
        obj.object_id,
        u64::from(obj.version),
        obj.digest
    )
}

impl Display for IotaGasData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Gas Owner: {}", self.owner)?;
        writeln!(f, "Gas Budget: {} NANOS", self.budget)?;
        writeln!(f, "Gas Price: {} NANOS", self.price)?;
        writeln!(f, "Gas Payment:")?;
        for payment in &self.payment {
            write!(f, "{} ", objref_string(payment))?;
        }
        writeln!(f)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaGenesisTransaction {
    pub objects: Vec<ObjectID>,
    pub events: Vec<EventID>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaConsensusCommitPrologueV1 {
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub round: u64,
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub sub_dag_index: Option<u64>,
    #[serde_as(as = "BigInt<u64>")]
    pub commit_timestamp_ms: u64,
    pub consensus_commit_digest: ConsensusCommitDigest,
    pub consensus_determined_version_assignments: ConsensusDeterminedVersionAssignments,
}

// #[serde_as]
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct IotaAuthenticatorStateUpdateV1 {
//     #[serde_as(as = "BigInt<u64>")]
//     pub epoch: u64,
//     #[serde_as(as = "BigInt<u64>")]
//     pub round: u64,

//     pub new_active_jwks: Vec<IotaActiveJwk>,
// }

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaRandomnessStateUpdate {
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: u64,

    #[serde_as(as = "BigInt<u64>")]
    pub randomness_round: u64,
    pub random_bytes: Vec<u8>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaEndOfEpochTransaction {
    pub transactions: Vec<IotaEndOfEpochTransactionKind>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaChangeEpoch {
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
    #[serde_as(as = "BigInt<u64>")]
    pub storage_charge: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub computation_charge: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub storage_rebate: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub epoch_start_timestamp_ms: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IotaEndOfEpochTransactionKind {
    ChangeEpoch(IotaChangeEpoch),
    ChangeEpochV2(IotaChangeEpochV2),
    AuthenticatorStateCreate,
    AuthenticatorStateExpire(IotaAuthenticatorStateExpire),
    BridgeStateCreate(CheckpointDigest),
    BridgeCommitteeUpdate(SequenceNumber),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaChangeEpochV2 {
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
    #[serde_as(as = "BigInt<u64>")]
    pub storage_charge: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub computation_charge: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub computation_charge_burned: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub storage_rebate: u64,
    #[serde_as(as = "BigInt<u64>")]
    pub epoch_start_timestamp_ms: u64,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaAuthenticatorStateExpire {
    #[serde_as(as = "BigInt<u64>")]
    pub min_epoch: u64,
}

// #[serde_as]
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct IotaActiveJwk {
//     pub jwk_id: IotaJwkId,
//     pub jwk: IotaJWK,

//     #[serde_as(as = "BigInt<u64>")]
//     pub epoch: u64,
// }

// impl From<ActiveJwk> for IotaActiveJwk {
//     fn from(active_jwk: ActiveJwk) -> Self {
//         Self {
//             jwk_id: IotaJwkId {
//                 iss: active_jwk.jwk_id.iss.clone(),
//                 kid: active_jwk.jwk_id.kid.clone(),
//             },
//             jwk: IotaJWK {
//                 kty: active_jwk.jwk.kty.clone(),
//                 e: active_jwk.jwk.e.clone(),
//                 n: active_jwk.jwk.n.clone(),
//                 alg: active_jwk.jwk.alg.clone(),
//             },
//             epoch: active_jwk.epoch,
//         }
//     }
// }

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaJwkId {
    pub iss: String,
    pub kid: String,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaJWK {
    pub kty: String,
    pub e: String,
    pub n: String,
    pub alg: String,
}

// /// A series of commands where the results of one command can be used in future
// /// commands
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
// pub struct IotaProgrammableTransactionBlock {
//     /// Input objects or primitive values
//     pub inputs: Vec<IotaCallArg>,
//     #[serde(rename = "transactions")]
//     /// The transactions to be executed sequentially. A failure in any
//     /// transaction will result in the failure of the entire programmable
//     /// transaction block.
//     pub commands: Vec<IotaCommand>,
// }

/// A single transaction in a programmable transaction block.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename = "IotaTransaction")]
pub enum IotaCommand {
    /// A call to either an entry or a public Move function
    MoveCall(Box<IotaProgrammableMoveCall>),
    /// `(Vec<forall T:key+store. T>, address)`
    /// It sends n-objects to the specified address. These objects must have
    /// store (public transfer) and either the previous owner must be an
    /// address or the object must be newly created.
    TransferObjects(Vec<IotaArgument>, IotaArgument),
    /// `(&mut Coin<T>, Vec<u64>)` -> `Vec<Coin<T>>`
    /// It splits off some amounts into a new coins with those amounts
    SplitCoins(IotaArgument, Vec<IotaArgument>),
    /// `(&mut Coin<T>, Vec<Coin<T>>)`
    /// It merges n-coins into the first coin
    MergeCoins(IotaArgument, Vec<IotaArgument>),
    /// Publishes a Move package. It takes the package bytes and a list of the
    /// package's transitive dependencies to link against on-chain.
    Publish(Vec<ObjectID>),
    /// Upgrades a Move package
    Upgrade(Vec<ObjectID>, ObjectID, IotaArgument),
    /// `forall T: Vec<T> -> vector<T>`
    /// Given n-values of the same type, it constructs a vector. For non objects
    /// or an empty vector, the type tag must be specified.
    MakeMoveVec(Option<String>, Vec<IotaArgument>),
}

/// An argument to a transaction in a programmable transaction block
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IotaArgument {
    /// The gas coin. The gas coin can only be used by-ref, except for with
    /// `TransferObjects`, which can use it by-value.
    GasCoin,
    /// One of the input objects or primitive values (from
    /// `ProgrammableTransactionBlock` inputs)
    Input(u16),
    /// The result of another transaction (from `ProgrammableTransactionBlock`
    /// transactions)
    Result(u16),
    /// Like a `Result` but it accesses a nested result. Currently, the only
    /// usage of this is to access a value from a Move call with multiple
    /// return values.
    NestedResult(u16, u16),
}

impl Display for IotaArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GasCoin => write!(f, "GasCoin"),
            Self::Input(i) => write!(f, "Input({i})"),
            Self::Result(i) => write!(f, "Result({i})"),
            Self::NestedResult(i, j) => write!(f, "NestedResult({i},{j})"),
        }
    }
}

/// The transaction for calling a Move function, either an entry function or a
/// public function (which cannot return references).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IotaProgrammableMoveCall {
    /// The package containing the module and function.
    pub package: ObjectID,
    /// The specific module in the package containing the function.
    pub module: String,
    /// The function to be called.
    pub function: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The type arguments to the function.
    pub type_arguments: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    /// The arguments to the function.
    pub arguments: Vec<IotaArgument>,
}

// #[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "camelCase")]
// pub enum IotaCallArg {
//     // Needs to become an Object Ref or Object ID, depending on object type
//     Object(IotaObjectArg),
//     // pure value, bcs encoded
//     Pure(IotaPureValue),
// }
// IotaTypeTag as AsIotaTypeTag,
// #[serde_as]
// #[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct IotaPureValue {
//     #[serde_as(as = "Option<AsIotaTypeTag>")]
//     value_type: Option<TypeTag>,
//     value: IotaJsonValue,
// }

// impl IotaPureValue {
//     pub fn value(&self) -> IotaJsonValue {
//         self.value.clone()
//     }

//     pub fn value_type(&self) -> Option<TypeTag> {
//         self.value_type.clone()
//     }
// }

// #[serde_as]
// #[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "objectType", rename_all = "camelCase")]
// pub enum IotaObjectArg {
//     // A Move object, either immutable, or owned mutable.
//     #[serde(rename_all = "camelCase")]
//     ImmOrOwnedObject {
//         object_id: ObjectID,
//         #[serde_as(as = "AsSequenceNumber")]
//         version: SequenceNumber,
//         digest: ObjectDigest,
//     },
//     // A Move object that's shared.
//     // SharedObject::mutable controls whether caller asks for a mutable reference to shared
//     // object.
//     #[serde(rename_all = "camelCase")]
//     SharedObject {
//         object_id: ObjectID,
//         #[serde_as(as = "AsSequenceNumber")]
//         initial_shared_version: SequenceNumber,
//         mutable: bool,
//     },
//     // A reference to a Move object that's going to be received in the transaction.
//     #[serde(rename_all = "camelCase")]
//     Receiving {
//         object_id: ObjectID,
//         #[serde_as(as = "AsSequenceNumber")]
//         version: SequenceNumber,
//         digest: ObjectDigest,
//     },
// }
