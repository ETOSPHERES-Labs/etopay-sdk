use crate::wallet::rebased::IotaResult;
//use crate::wallet::rebased::MoveValue;

use super::IotaMoveStruct;
use super::IotaMoveValue;
use super::StructTag;
use super::{ObjectDigest, ObjectID, ObjectRef, SequenceNumber};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "camelCase", rename = "ObjectRef")]
pub struct IotaObjectRef {
    /// Hex code as string representing the object id
    pub object_id: ObjectID,
    /// Object version.
    pub version: SequenceNumber,
    /// Base64 string representing the object digest
    pub digest: ObjectDigest,
}

impl IotaObjectRef {
    pub fn to_object_ref(&self) -> ObjectRef {
        (self.object_id, self.version, self.digest)
    }
}

impl Display for IotaObjectRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Object ID: {}, version: {}, digest: {}",
            self.object_id, self.version, self.digest
        )
    }
}

impl From<ObjectRef> for IotaObjectRef {
    fn from(oref: ObjectRef) -> Self {
        Self {
            object_id: oref.0,
            version: oref.1,
            digest: oref.2,
        }
    }
}

// pub fn type_and_fields_from_move_event_data(event_data: MoveValue) -> IotaResult<(StructTag, serde_json::Value)> {
//     match event_data.into() {
//         IotaMoveValue::Struct(move_struct) => match &move_struct {
//             IotaMoveStruct::WithTypes { type_, .. } => Ok((type_.clone(), move_struct.clone().to_json_value())),
//             _ => Err(crate::Error::ObjectDeserialization {
//                 error: "Found non-type IotaMoveStruct in MoveValue event".to_string(),
//             }),
//         },
//         IotaMoveValue::Variant(v) => Ok((v.type_.clone(), v.clone().to_json_value())),
//         IotaMoveValue::Vector(_)
//         | IotaMoveValue::Number(_)
//         | IotaMoveValue::Bool(_)
//         | IotaMoveValue::Address(_)
//         | IotaMoveValue::String(_)
//         | IotaMoveValue::UID { .. }
//         | IotaMoveValue::Option(_) => Err(crate::Error::ObjectDeserialization {
//             error: "Invalid MoveValue event type -- this should not be possible".to_string(),
//         }),
//     }
// }
