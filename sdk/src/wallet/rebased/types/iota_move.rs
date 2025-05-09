use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use serde_with::serde_as;

use crate::wallet::rebased::{language_storage_min::StructTag, serde::IotaStructTag};

use super::{IotaAddress, ObjectID};

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveValue")]
pub enum IotaMoveValue {
    // u64 and u128 are converted to String to avoid overflow
    Number(u32),
    Bool(bool),
    Address(IotaAddress),
    Vector(Vec<IotaMoveValue>),
    String(String),
    UID { id: ObjectID },
    Struct(IotaMoveStruct),
    Option(Box<Option<IotaMoveValue>>),
    Variant(IotaMoveVariant),
}

impl IotaMoveValue {
    /// Extract values from MoveValue without type information in json format
    pub fn to_json_value(self) -> Value {
        match self {
            IotaMoveValue::Struct(move_struct) => move_struct.to_json_value(),
            IotaMoveValue::Vector(values) => IotaMoveStruct::Runtime(values).to_json_value(),
            IotaMoveValue::Number(v) => json!(v),
            IotaMoveValue::Bool(v) => json!(v),
            IotaMoveValue::Address(v) => json!(v),
            IotaMoveValue::String(v) => json!(v),
            IotaMoveValue::UID { id } => json!({ "id": id }),
            IotaMoveValue::Option(v) => json!(v),
            IotaMoveValue::Variant(v) => v.to_json_value(),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(untagged, rename = "MoveStruct")]
pub enum IotaMoveStruct {
    Runtime(Vec<IotaMoveValue>),
    WithTypes {
        #[serde(rename = "type")]
        #[serde_as(as = "IotaStructTag")]
        type_: StructTag,
        fields: BTreeMap<String, IotaMoveValue>,
    },
    WithFields(BTreeMap<String, IotaMoveValue>),
}

impl IotaMoveStruct {
    /// Extract values from MoveStruct without type information in json format
    pub fn to_json_value(self) -> Value {
        // Unwrap MoveStructs
        match self {
            IotaMoveStruct::Runtime(values) => {
                let values = values
                    .into_iter()
                    .map(|value| value.to_json_value())
                    .collect::<Vec<_>>();
                json!(values)
            }
            // We only care about values here, assuming struct type information is known at the
            // client side.
            IotaMoveStruct::WithTypes { type_: _, fields } | IotaMoveStruct::WithFields(fields) => {
                let fields = fields
                    .into_iter()
                    .map(|(key, value)| (key, value.to_json_value()))
                    .collect::<BTreeMap<_, _>>();
                json!(fields)
            }
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone, Eq, PartialEq)]
#[serde(rename = "MoveVariant")]
pub struct IotaMoveVariant {
    #[serde(rename = "type")]
    #[serde_as(as = "IotaStructTag")]
    pub type_: StructTag,
    pub variant: String,
    pub fields: BTreeMap<String, IotaMoveValue>,
}

impl IotaMoveVariant {
    pub fn to_json_value(self) -> Value {
        // We only care about values here, assuming type information is known at the
        // client side.
        let fields = self
            .fields
            .into_iter()
            .map(|(key, value)| (key, value.to_json_value()))
            .collect::<BTreeMap<_, _>>();
        json!({
            "variant": self.variant,
            "fields": fields,
        })
    }
}
