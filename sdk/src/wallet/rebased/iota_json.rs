use super::{IotaAddress, ObjectID};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value as JsonValue, json};

use std::{
    collections::{BTreeMap, VecDeque},
    fmt::{self, Debug, Formatter},
    str::FromStr,
};

#[derive(Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct IotaJsonValue(JsonValue);
impl IotaJsonValue {
    pub fn new(json_value: JsonValue) -> Result<IotaJsonValue, anyhow::Error> {
        Self::check_value(&json_value)?;
        Ok(Self(json_value))
    }

    fn check_value(json_value: &JsonValue) -> Result<(), anyhow::Error> {
        match json_value {
            // No checks needed for Bool and String
            JsonValue::Bool(_) | JsonValue::String(_) => (),
            JsonValue::Number(n) => {
                // Must be castable to u64
                if !n.is_u64() {
                    return Err(anyhow!(
                        "{n} not allowed. Number must be unsigned integer of at most u32"
                    ));
                }
            }
            // Must be homogeneous
            JsonValue::Array(a) => {
                // Fail if not homogeneous
                check_valid_homogeneous(&JsonValue::Array(a.to_vec()))?
            }
            JsonValue::Object(v) => {
                for (_, value) in v {
                    Self::check_value(value)?;
                }
            }
            JsonValue::Null => bail!("Null not allowed."),
        };
        Ok(())
    }

    pub fn from_object_id(id: ObjectID) -> IotaJsonValue {
        Self(JsonValue::String(id.to_hex_uncompressed()))
    }

    // pub fn to_bcs_bytes(&self, ty: &MoveTypeLayout) -> Result<Vec<u8>, anyhow::Error> {
    //     let move_value = Self::to_move_value(&self.0, ty)?;
    //     R::MoveValue::simple_serialize(&move_value)
    //         .ok_or_else(|| anyhow!("Unable to serialize {:?}. Expected {}", move_value, ty))
    // }

    // pub fn from_bcs_bytes(layout: Option<&MoveTypeLayout>, bytes: &[u8]) -> Result<Self, anyhow::Error> {
    //     let json = if let Some(layout) = layout {
    //         // Try to convert Vec<u8> inputs into string
    //         fn try_parse_string(layout: &MoveTypeLayout, bytes: &[u8]) -> Option<String> {
    //             if let MoveTypeLayout::Vector(t) = layout {
    //                 if let MoveTypeLayout::U8 = **t {
    //                     return bcs::from_bytes::<String>(bytes).ok();
    //                 }
    //             }
    //             None
    //         }
    //         if let Some(s) = try_parse_string(layout, bytes) {
    //             json!(s)
    //         } else {
    //             let result = BoundedVisitor::deserialize_value(bytes, layout).map_or_else(
    //                 |_| {
    //                     // fallback to array[u8] if fail to convert to json.
    //                     JsonValue::Array(bytes.iter().map(|b| JsonValue::Number(Number::from(*b))).collect())
    //                 },
    //                 |move_value| {
    //                     move_value_to_json(&move_value).unwrap_or_else(|| {
    //                         // fallback to array[u8] if fail to convert to json.
    //                         JsonValue::Array(bytes.iter().map(|b| JsonValue::Number(Number::from(*b))).collect())
    //                     })
    //                 },
    //             );
    //             result
    //         }
    //     } else {
    //         json!(bytes)
    //     };
    //     IotaJsonValue::new(json)
    // }

    pub fn to_json_value(&self) -> JsonValue {
        self.0.clone()
    }

    pub fn to_iota_address(&self) -> anyhow::Result<IotaAddress> {
        json_value_to_iota_address(&self.0)
    }

    // fn handle_inner_struct_layout(
    //     inner_vec: &[MoveFieldLayout],
    //     val: &JsonValue,
    //     ty: &MoveTypeLayout,
    //     s: &String,
    // ) -> Result<R::MoveValue, anyhow::Error> {
    //     // delegate MoveValue construction to the case when JsonValue::String and
    //     // MoveTypeLayout::Vector are handled to get an address (with 0x string
    //     // prefix) or a vector of u8s (no prefix)
    //     debug_assert!(matches!(val, JsonValue::String(_)));

    //     if inner_vec.len() != 1 {
    //         bail!(
    //             "Cannot convert string arg {s} to {ty} which is expected \
    //              to be a struct with one field"
    //         );
    //     }

    //     match &inner_vec[0].layout {
    //         MoveTypeLayout::Vector(inner) => match **inner {
    //             MoveTypeLayout::U8 => Ok(R::MoveValue::Struct(R::MoveStruct(vec![Self::to_move_value(
    //                 val,
    //                 &inner_vec[0].layout.clone(),
    //             )?]))),
    //             MoveTypeLayout::Address => Ok(R::MoveValue::Struct(R::MoveStruct(vec![Self::to_move_value(
    //                 val,
    //                 &MoveTypeLayout::Address,
    //             )?]))),
    //             _ => bail!(
    //                 "Cannot convert string arg {s} to {ty} \
    //                          which is expected to be a struct \
    //                          with one field of address or u8 vector type"
    //             ),
    //         },
    //         MoveTypeLayout::Struct(struct_layout) if struct_layout.type_ == ID::type_() => Ok(R::MoveValue::Struct(
    //             R::MoveStruct(vec![Self::to_move_value(val, &inner_vec[0].layout.clone())?]),
    //         )),
    //         _ => bail!(
    //             "Cannot convert string arg {s} to {ty} which is expected \
    //              to be a struct with one field of a vector type"
    //         ),
    //     }
    // }

    // pub fn to_move_value(val: &JsonValue, ty: &MoveTypeLayout) -> Result<R::MoveValue, anyhow::Error> {
    //     Ok(match (val, ty) {
    //         // Bool to Bool is simple
    //         (JsonValue::Bool(b), MoveTypeLayout::Bool) => R::MoveValue::Bool(*b),

    //         // In constructor, we have already checked that the JSON number is unsigned int of at
    //         // most U32
    //         (JsonValue::Number(n), MoveTypeLayout::U8) => match n.as_u64() {
    //             Some(x) => R::MoveValue::U8(u8::try_from(x)?),
    //             None => return Err(anyhow!("{} is not a valid number. Only u8 allowed.", n)),
    //         },
    //         (JsonValue::Number(n), MoveTypeLayout::U16) => match n.as_u64() {
    //             Some(x) => R::MoveValue::U16(u16::try_from(x)?),
    //             None => return Err(anyhow!("{} is not a valid number. Only u16 allowed.", n)),
    //         },
    //         (JsonValue::Number(n), MoveTypeLayout::U32) => match n.as_u64() {
    //             Some(x) => R::MoveValue::U32(u32::try_from(x)?),
    //             None => return Err(anyhow!("{} is not a valid number. Only u32 allowed.", n)),
    //         },

    //         // u8, u16, u32, u64, u128, u256 can be encoded as String
    //         (JsonValue::String(s), MoveTypeLayout::U8) => {
    //             R::MoveValue::U8(u8::try_from(convert_string_to_u256(s.as_str())?)?)
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::U16) => {
    //             R::MoveValue::U16(u16::try_from(convert_string_to_u256(s.as_str())?)?)
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::U32) => {
    //             R::MoveValue::U32(u32::try_from(convert_string_to_u256(s.as_str())?)?)
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::U64) => {
    //             R::MoveValue::U64(u64::try_from(convert_string_to_u256(s.as_str())?)?)
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::U128) => {
    //             R::MoveValue::U128(u128::try_from(convert_string_to_u256(s.as_str())?)?)
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::U256) => R::MoveValue::U256(convert_string_to_u256(s.as_str())?),
    //         // For ascii and utf8 strings
    //         (JsonValue::String(s), MoveTypeLayout::Struct(struct_layout))
    //             if is_move_string_type(&struct_layout.type_) =>
    //         {
    //             R::MoveValue::Vector(s.as_bytes().iter().copied().map(R::MoveValue::U8).collect())
    //         }
    //         // For ID
    //         (JsonValue::String(s), MoveTypeLayout::Struct(struct_layout)) if struct_layout.type_ == ID::type_() => {
    //             if struct_layout.fields.len() != 1 {
    //                 bail!(
    //                     "Cannot convert string arg {s} to {} which is expected to be a struct with one field",
    //                     struct_layout.type_
    //                 );
    //             };
    //             let addr = IotaAddress::from_str(s)?;
    //             R::MoveValue::Address(addr.into())
    //         }
    //         (JsonValue::Object(o), MoveTypeLayout::Struct(struct_layout)) => {
    //             let mut field_values = vec![];
    //             for layout in struct_layout.fields.iter() {
    //                 let field = o
    //                     .get(layout.name.as_str())
    //                     .ok_or_else(|| anyhow!("Missing field {} for struct {ty}", layout.name))?;
    //                 field_values.push(Self::to_move_value(field, &layout.layout)?);
    //             }
    //             R::MoveValue::Struct(R::MoveStruct(field_values))
    //         }
    //         // Unnest fields
    //         (value, MoveTypeLayout::Struct(struct_layout)) if struct_layout.fields.len() == 1 => {
    //             Self::to_move_value(value, &struct_layout.fields[0].layout)?
    //         }
    //         (JsonValue::String(s), MoveTypeLayout::Vector(t)) => {
    //             match &**t {
    //                 MoveTypeLayout::U8 => {
    //                     // We can encode U8 Vector as string in 2 ways
    //                     // 1. If it starts with 0x, we treat it as hex strings, where each pair is a
    //                     //    byte
    //                     // 2. If it does not start with 0x, we treat each character as an ASCII
    //                     //    encoded byte
    //                     // We have to support both for the convenience of the user. This is because
    //                     // sometime we need Strings as arg Other times we need vec of hex bytes for
    //                     // address. Issue is both Address and Strings are represented as Vec<u8> in
    //                     // Move call
    //                     let vec = if s.starts_with(HEX_PREFIX) {
    //                         // If starts with 0x, treat as hex vector
    //                         Hex::decode(s).map_err(|e| anyhow!(e))?
    //                     } else {
    //                         // Else raw bytes
    //                         s.as_bytes().to_vec()
    //                     };
    //                     R::MoveValue::Vector(vec.iter().copied().map(R::MoveValue::U8).collect())
    //                 }
    //                 MoveTypeLayout::Struct(struct_layout) => {
    //                     Self::handle_inner_struct_layout(&struct_layout.fields, val, ty, s)?
    //                 }
    //                 _ => bail!("Cannot convert string arg {s} to {ty}"),
    //             }
    //         }

    //         // We have already checked that the array is homogeneous in the constructor
    //         (JsonValue::Array(a), MoveTypeLayout::Vector(inner)) => {
    //             // Recursively build an IntermediateValue array
    //             R::MoveValue::Vector(
    //                 a.iter()
    //                     .map(|i| Self::to_move_value(i, inner))
    //                     .collect::<Result<Vec<_>, _>>()?,
    //             )
    //         }

    //         (v, MoveTypeLayout::Address) => {
    //             let addr = json_value_to_iota_address(v)?;
    //             R::MoveValue::Address(addr.into())
    //         }

    //         _ => bail!("Unexpected arg {val:?} for expected type {ty:?}"),
    //     })
    // }
}

impl Debug for IotaJsonValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

const HEX_PREFIX: &str = "0x";

fn json_value_to_iota_address(value: &JsonValue) -> anyhow::Result<IotaAddress> {
    match value {
        JsonValue::String(s) => {
            let s = s.trim().to_lowercase();
            if !s.starts_with(HEX_PREFIX) {
                bail!("Address hex string must start with 0x.",);
            }
            Ok(IotaAddress::from_str(&s)?)
        }
        JsonValue::Array(bytes) => {
            fn value_to_byte_array(v: &Vec<JsonValue>) -> Option<Vec<u8>> {
                let mut bytes = vec![];
                for b in v {
                    let b = b.as_u64()?;
                    if b <= u8::MAX as u64 {
                        bytes.push(b as u8);
                    } else {
                        return None;
                    }
                }
                Some(bytes)
            }
            let bytes = value_to_byte_array(bytes)
                .ok_or_else(|| anyhow!("Invalid input: Cannot parse input into IotaAddress."))?;
            Ok(IotaAddress::try_from(bytes)?)
        }
        v => bail!("Unexpected arg {v} for expected type address"),
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum ValidJsonType {
    Bool,
    Number,
    String,
    Array,
    // Matches any type
    Any,
}

/// Check via BFS
/// The invariant is that all types at a given level must be the same or be
/// empty, and all must be valid
pub fn check_valid_homogeneous(val: &JsonValue) -> Result<(), IotaJsonValueError> {
    let mut deq: VecDeque<&JsonValue> = VecDeque::new();
    deq.push_back(val);
    check_valid_homogeneous_rec(&mut deq)
}

/// Check via BFS
/// The invariant is that all types at a given level must be the same or be
/// empty
fn check_valid_homogeneous_rec(curr_q: &mut VecDeque<&JsonValue>) -> Result<(), IotaJsonValueError> {
    if curr_q.is_empty() {
        // Nothing to do
        return Ok(());
    }
    // Queue for the next level
    let mut next_q = VecDeque::new();
    // The types at this level must be the same
    let mut level_type = ValidJsonType::Any;

    // Process all in this queue/level
    while let Some(v) = curr_q.pop_front() {
        let curr = match v {
            JsonValue::Bool(_) => ValidJsonType::Bool,
            JsonValue::Number(x) if x.is_u64() => ValidJsonType::Number,
            JsonValue::String(_) => ValidJsonType::String,
            JsonValue::Array(w) => {
                // Add to the next level
                w.iter().for_each(|t| next_q.push_back(t));
                ValidJsonType::Array
            }
            // Not valid
            _ => {
                return Err(IotaJsonValueError::new(v, IotaJsonValueErrorKind::ValueTypeNotAllowed));
            }
        };

        if level_type == ValidJsonType::Any {
            // Update the level with the first found type
            level_type = curr;
        } else if level_type != curr {
            // Mismatch in the level
            return Err(IotaJsonValueError::new(v, IotaJsonValueErrorKind::ArrayNotHomogeneous));
        }
    }
    // Process the next level
    check_valid_homogeneous_rec(&mut next_q)
}

/// A list of error categories encountered when parsing numbers.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IotaJsonValueErrorKind {
    /// JSON value must be of specific types.
    ValueTypeNotAllowed,

    /// JSON arrays must be homogeneous.
    ArrayNotHomogeneous,
}

#[derive(Debug)]
pub struct IotaJsonValueError {
    kind: IotaJsonValueErrorKind,
    val: JsonValue,
}

impl IotaJsonValueError {
    pub fn new(val: &JsonValue, kind: IotaJsonValueErrorKind) -> Self {
        Self { kind, val: val.clone() }
    }
}

impl std::error::Error for IotaJsonValueError {}

impl fmt::Display for IotaJsonValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_str = match self.kind {
            IotaJsonValueErrorKind::ValueTypeNotAllowed => {
                format!("JSON value type {} not allowed.", self.val)
            }
            IotaJsonValueErrorKind::ArrayNotHomogeneous => {
                format!("Array not homogeneous. Mismatched value: {}.", self.val)
            }
        };
        write!(f, "{err_str}")
    }
}
