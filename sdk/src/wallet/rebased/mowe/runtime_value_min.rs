use digest::consts::U256;
use serde::{
    Deserialize, Serialize,
    de::{Error as DeError, Visitor},
    ser::{SerializeSeq, SerializeTuple},
};
use std::{
    fmt::{self, Debug},
    io::Cursor,
};

use crate::wallet::rebased::{AccountAddress, error::Result};

use super::annotated_value_min::MoveTypeLayout;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveStruct(pub Vec<MoveValue>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MoveVariant {
    pub tag: u16,
    pub fields: Vec<MoveValue>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MoveValue {
    U8(u8),
    U64(u64),
    U128(u128),
    Bool(bool),
    Address(AccountAddress),
    Vector(Vec<MoveValue>),
    Struct(MoveStruct),
    Signer(AccountAddress),
    // NOTE: Added in bytecode version v6, do not reorder!
    U16(u16),
    U32(u32),
    U256(U256),
    Variant(MoveVariant),
}

impl MoveValue {
    /// Deserialize `blob` as a Move value with the given `ty`-pe layout, and
    /// visit its sub-structure with the given `visitor`. The visitor
    /// dictates the return value that is built up during deserialization.
    ///
    /// # Nested deserialization
    ///
    /// Vectors and structs are nested structures that can be met during
    /// deserialization. Visitors are passed a driver (`VecDriver` or
    /// `StructDriver` correspondingly) which controls how nested elements
    /// or fields are visited including whether a given nested element/field is
    /// explored, which visitor to use (the visitor can pass `self` to
    /// recursively explore them) and whether a given element is visited or
    /// skipped.
    ///
    /// The visitor may leave elements unvisited at the end of the vector or
    /// struct, which implicitly skips them.
    ///
    /// # Errors
    ///
    /// Deserialization can fail because of an issue in the serialized format
    /// (data doesn't match layout, unexpected bytes or trailing bytes), or
    /// a custom error expressed by the visitor.
    pub fn visit_deserialize<'b, 'l, V: Visitor<'b, 'l>>(
        blob: &'b [u8],
        ty: &'l MoveTypeLayout,
        visitor: &mut V,
    ) -> Result<V::Value>
    where
        V::Error: std::error::Error + Send + Sync + 'static,
    {
        let mut bytes = Cursor::new(blob);
        // annotated_visitor::{Error as VError, ValueDriver, Visitor, visit_struct, visit_value},
        let res = visit_value(&mut bytes, ty, visitor)?;
        if bytes.position() as usize == blob.len() {
            Ok(res)
        } else {
            let remaining = blob.len() - bytes.position() as usize;
            Err(VError::TrailingBytes(remaining).into())
        }
    }
}
