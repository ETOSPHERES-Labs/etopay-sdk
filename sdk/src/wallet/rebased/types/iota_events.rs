use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use serde_with::{DisplayFromStr, serde_as};

use crate::wallet::rebased::IotaResult;
use crate::wallet::rebased::annotated_value_min::MoveDatatypeLayout;
use crate::wallet::rebased::error::Result;
use crate::wallet::rebased::language_storage_min::StructTag;
use crate::wallet::rebased::runtime_value_min::MoveStruct;
use crate::wallet::rebased::runtime_value_min::MoveValue;
use crate::wallet::rebased::runtime_value_min::MoveVariant;
// use crate::wallet::rebased::annotated_value_min::MoveValue;
// use crate::wallet::rebased::MoveDatatypeLayout;
use crate::wallet::rebased::Page;
use crate::wallet::rebased::bigint::BigInt;
use crate::wallet::rebased::encoding::Base58;
use crate::wallet::rebased::encoding::Base64;
use crate::wallet::rebased::serde::IotaStructTag;

use super::Event;
use super::IotaMoveStruct;
use super::IotaMoveValue;
use super::IotaMoveVariant;
use super::TransactionDigest;
// use super::type_and_fields_from_move_event_data;
use super::{EventID, Identifier, IotaAddress, ObjectID};

pub type EventPage = Page<IotaEvent, EventID>;

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename = "Event", rename_all = "camelCase")]
pub struct IotaEvent {
    /// Sequential event ID, ie (transaction seq number, event seq number).
    /// 1) Serves as a unique event ID for each fullnode
    /// 2) Also serves to sequence events for the purposes of pagination and
    ///    querying. A higher id is an event seen later by that fullnode.
    /// This ID is the "cursor" for event querying.
    pub id: EventID,
    /// Move package where this event was emitted.
    pub package_id: ObjectID,
    #[serde_as(as = "DisplayFromStr")]
    /// Move module where this event was emitted.
    pub transaction_module: Identifier,
    /// Sender's IOTA address.
    pub sender: IotaAddress,
    #[serde_as(as = "IotaStructTag")]
    /// Move event type.
    pub type_: StructTag,
    /// Parsed json value of the event
    pub parsed_json: Value,
    /// Base64 encoded bcs bytes of the move event
    #[serde(flatten)]
    pub bcs: BcsEvent,
    /// UTC timestamp in milliseconds since epoch (1/1/1970)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde_as(as = "Option<BigInt<u64>>")]
    pub timestamp_ms: Option<u64>,
}

impl From<IotaEvent> for Event {
    fn from(val: IotaEvent) -> Self {
        Event {
            package_id: val.package_id,
            transaction_module: val.transaction_module,
            sender: val.sender,
            type_: val.type_,
            contents: val.bcs.into_bytes(),
        }
    }
}

impl IotaEvent {
    pub fn try_from(
        event: Event,
        tx_digest: TransactionDigest,
        event_seq: u64,
        timestamp_ms: Option<u64>,
        layout: MoveDatatypeLayout,
    ) -> IotaResult<Self> {
        let Event {
            package_id,
            transaction_module,
            sender,
            type_: _,
            contents,
        } = event;

        let bcs = BcsEvent::Base64 { bcs: contents.to_vec() };

        let move_value = Event::move_event_to_move_value(&contents, layout)?;
        let (type_, fields) = type_and_fields_from_move_event_data(move_value)?;

        Ok(IotaEvent {
            id: EventID { tx_digest, event_seq },
            package_id,
            transaction_module,
            sender,
            type_,
            parsed_json: fields,
            bcs,
            timestamp_ms,
        })
    }
}

#[serde_as]
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "bcsEncoding")]
#[serde(from = "MaybeTaggedBcsEvent")]
pub enum BcsEvent {
    Base64 {
        #[serde_as(as = "Base64")]
        bcs: Vec<u8>,
    },
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

impl BcsEvent {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self::Base64 { bcs: bytes }
    }

    pub fn bytes(&self) -> &[u8] {
        match self {
            BcsEvent::Base64 { bcs } => bcs.as_ref(),
            BcsEvent::Base58 { bcs } => bcs.as_ref(),
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            BcsEvent::Base64 { bcs } => bcs,
            BcsEvent::Base58 { bcs } => bcs,
        }
    }
}

#[allow(unused)]
#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
enum MaybeTaggedBcsEvent {
    Tagged(TaggedBcsEvent),
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "bcsEncoding")]
enum TaggedBcsEvent {
    Base64 {
        #[serde_as(as = "Base64")]
        bcs: Vec<u8>,
    },
    Base58 {
        #[serde_as(as = "Base58")]
        bcs: Vec<u8>,
    },
}

impl From<MaybeTaggedBcsEvent> for BcsEvent {
    fn from(event: MaybeTaggedBcsEvent) -> BcsEvent {
        let bcs = match event {
            MaybeTaggedBcsEvent::Tagged(TaggedBcsEvent::Base58 { bcs }) | MaybeTaggedBcsEvent::Base58 { bcs } => bcs,
            MaybeTaggedBcsEvent::Tagged(TaggedBcsEvent::Base64 { bcs }) => bcs,
        };

        // Bytes are already decoded, force into Base64 variant to avoid serializing to
        // base58
        Self::Base64 { bcs }
    }
}

pub fn type_and_fields_from_move_event_data(event_data: MoveValue) -> Result<(StructTag, serde_json::Value)> {
    match event_data.into() {
        IotaMoveValue::Struct(move_struct) => match &move_struct {
            IotaMoveStruct::WithTypes { type_, .. } => Ok((type_.clone(), move_struct.clone().to_json_value())),
            _ => Err(crate::wallet::rebased::error::RebasedError::ObjectDeserialization {
                error: "Found non-type IotaMoveStruct in MoveValue event".to_string(),
            }),
        },
        IotaMoveValue::Variant(v) => Ok((v.type_.clone(), v.clone().to_json_value())),
        IotaMoveValue::Vector(_)
        | IotaMoveValue::Number(_)
        | IotaMoveValue::Bool(_)
        | IotaMoveValue::Address(_)
        | IotaMoveValue::String(_)
        | IotaMoveValue::UID { .. }
        | IotaMoveValue::Option(_) => Err(crate::wallet::rebased::error::RebasedError::ObjectDeserialization {
            error: "Invalid MoveValue event type -- this should not be possible".to_string(),
        }),
    }
}

impl From<MoveValue> for IotaMoveValue {
    fn from(value: MoveValue) -> Self {
        match value {
            MoveValue::U8(value) => IotaMoveValue::Number(value.into()),
            MoveValue::U16(value) => IotaMoveValue::Number(value.into()),
            MoveValue::U32(value) => IotaMoveValue::Number(value),
            MoveValue::U64(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::U128(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::U256(value) => IotaMoveValue::String(format!("{value}")),
            MoveValue::Bool(value) => IotaMoveValue::Bool(value),
            MoveValue::Vector(values) => IotaMoveValue::Vector(values.into_iter().map(|value| value.into()).collect()),
            MoveValue::Struct(value) => {
                // Best effort IOTA core type conversion
                let MoveStruct { type_, fields } = &value;
                if let Some(value) = try_convert_type(type_, fields) {
                    return value;
                }
                IotaMoveValue::Struct(value.into())
            }
            MoveValue::Signer(value) | MoveValue::Address(value) => {
                IotaMoveValue::Address(IotaAddress::from(ObjectID::from(value)))
            }
            MoveValue::Variant(MoveVariant {
                type_,
                variant_name,
                tag: _,
                fields,
            }) => IotaMoveValue::Variant(IotaMoveVariant {
                type_: type_.clone(),
                variant: variant_name.to_string(),
                fields: fields
                    .into_iter()
                    .map(|(id, value)| (id.into_string(), value.into()))
                    .collect::<BTreeMap<_, _>>(),
            }),
        }
    }
}

fn try_convert_type(type_: &StructTag, fields: &[(Identifier, MoveValue)]) -> Option<IotaMoveValue> {
    let struct_name = format!(
        "0x{}::{}::{}",
        type_.address.short_str_lossless(),
        type_.module,
        type_.name
    );
    let mut values = fields
        .iter()
        .map(|(id, value)| (id.to_string(), value))
        .collect::<BTreeMap<_, _>>();
    match struct_name.as_str() {
        "0x1::string::String" | "0x1::ascii::String" => {
            if let Some(MoveValue::Vector(bytes)) = values.remove("bytes") {
                return to_bytearray(bytes)
                    .and_then(|bytes| String::from_utf8(bytes).ok())
                    .map(IotaMoveValue::String);
            }
        }
        "0x2::url::Url" => {
            return values.remove("url").cloned().map(IotaMoveValue::from);
        }
        "0x2::object::ID" => {
            return values.remove("bytes").cloned().map(IotaMoveValue::from);
        }
        "0x2::object::UID" => {
            let id = values.remove("id").cloned().map(IotaMoveValue::from);
            if let Some(IotaMoveValue::Address(address)) = id {
                return Some(IotaMoveValue::UID {
                    id: ObjectID::from(address),
                });
            }
        }
        "0x2::balance::Balance" => {
            return values.remove("value").cloned().map(IotaMoveValue::from);
        }
        "0x1::option::Option" => {
            if let Some(MoveValue::Vector(values)) = values.remove("vec") {
                return Some(IotaMoveValue::Option(Box::new(
                    // in Move option is modeled as vec of 1 element
                    values.first().cloned().map(IotaMoveValue::from),
                )));
            }
        }
        _ => return None,
    }
    warn!(
        fields =? fields,
        "Failed to convert {struct_name} to IotaMoveValue"
    );
    None
}

fn to_bytearray(value: &[MoveValue]) -> Option<Vec<u8>> {
    if value.iter().all(|value| matches!(value, MoveValue::U8(_))) {
        let bytearray = value
            .iter()
            .flat_map(|value| if let MoveValue::U8(u8) = value { Some(*u8) } else { None })
            .collect::<Vec<_>>();
        Some(bytearray)
    } else {
        None
    }
}
