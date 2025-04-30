use super::super::bigint::BigInt;
use super::super::serde::Readable;
use serde::{Deserialize, Serialize};
use serde_with::{Bytes, serde_as};
use std::str::FromStr;

use crate::wallet::rebased::RebasedError;

use super::{
    AccountAddress,
    IdentStr,
    Identifier,
    IotaAddress,
    ObjectID,
    StructTag,
    TransactionDigest,
    //iota_serde::{BigInt, Readable},
};

/// Specific type of event
#[serde_as]
#[derive(PartialEq, Eq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct Event {
    pub package_id: ObjectID,
    pub transaction_module: Identifier,
    pub sender: IotaAddress,
    pub type_: StructTag,
    #[serde_as(as = "Bytes")]
    pub contents: Vec<u8>,
}

impl Event {
    pub fn new(
        package_id: &AccountAddress,
        module: &IdentStr,
        sender: IotaAddress,
        type_: StructTag,
        contents: Vec<u8>,
    ) -> Self {
        Self {
            package_id: ObjectID::from(*package_id),
            transaction_module: Identifier::from(module),
            sender,
            type_,
            contents,
        }
    }
    // pub fn move_event_to_move_value(contents: &[u8], layout: MoveDatatypeLayout) -> IotaResult<MoveValue> {
    //     BoundedVisitor::deserialize_value(contents, &layout.into_layout())
    //         .map_err(|e| IotaError::ObjectSerialization { error: e.to_string() })
    // }

    // pub fn is_system_epoch_info_event_v1(&self) -> bool {
    //     self.type_.address == IOTA_SYSTEM_ADDRESS
    //         && self.type_.module.as_ident_str() == ident_str!("iota_system_state_inner")
    //         && self.type_.name.as_ident_str() == ident_str!("SystemEpochInfoEventV1")
    // }

    // pub fn is_system_epoch_info_event_v2(&self) -> bool {
    //     self.type_.address == IOTA_SYSTEM_ADDRESS
    //         && self.type_.module.as_ident_str() == ident_str!("iota_system_state_inner")
    //         && self.type_.name.as_ident_str() == ident_str!("SystemEpochInfoEventV2")
    // }

    // pub fn is_system_epoch_info_event(&self) -> bool {
    //     self.is_system_epoch_info_event_v1() || self.is_system_epoch_info_event_v2()
    // }
}

// impl Event {
//     pub fn random_for_testing() -> Self {
//         Self {
//             package_id: ObjectID::random(),
//             transaction_module: Identifier::new("test").unwrap(),
//             sender: AccountAddress::random().into(),
//             type_: StructTag {
//                 address: AccountAddress::random(),
//                 module: Identifier::new("test").unwrap(),
//                 name: Identifier::new("test").unwrap(),
//                 type_params: vec![],
//             },
//             contents: vec![],
//         }
//     }
// }

/// Unique ID of an IOTA Event, the ID is a combination of tx seq number and
/// event seq number, the ID is local to this particular fullnode and will be
/// different from other fullnode.
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "camelCase")]
pub struct EventID {
    pub tx_digest: TransactionDigest,
    #[serde_as(as = "Readable<BigInt<u64>, _>")]
    pub event_seq: u64,
}

impl From<(TransactionDigest, u64)> for EventID {
    fn from((tx_digest_num, event_seq_number): (TransactionDigest, u64)) -> Self {
        Self {
            tx_digest: tx_digest_num as TransactionDigest,
            event_seq: event_seq_number,
        }
    }
}

impl From<EventID> for String {
    fn from(id: EventID) -> Self {
        format!("{:?}:{}", id.tx_digest, id.event_seq)
    }
}

impl TryFrom<String> for EventID {
    type Error = RebasedError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let values = value.split(':').collect::<Vec<_>>();
        // use anyhow::ensure;
        // ensure!(values.len() == 2, "Malformed EventID : {value}");
        if values.len() != 2 {
            return Err(format!("Malformed EventID : {}", value));
        }

        Ok((TransactionDigest::from_str(values[0])?, u64::from_str(values[1])?).into())
    }
}
