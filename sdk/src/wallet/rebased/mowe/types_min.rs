use crate::wallet::rebased::{AccountAddress, Identifier, error::Result};

use super::{
    address_min::ParsedAddress,
    language_storage_min::{StructTag, TypeTag},
};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParsedType {
    U8,
    U16,
    U32,
    U64,
    U128,
    U256,
    Bool,
    Address,
    Signer,
    Vector(Box<ParsedType>),
    Struct(ParsedStructType),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedStructType {
    pub fq_name: ParsedFqName,
    pub type_args: Vec<ParsedType>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedModuleId {
    pub address: ParsedAddress,
    pub name: String,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct ParsedFqName {
    pub module: ParsedModuleId,
    pub name: String,
}

impl ParsedType {
    pub fn into_type_tag(self, mapping: &impl Fn(&str) -> Option<AccountAddress>) -> Result<TypeTag> {
        Ok(match self {
            ParsedType::U8 => TypeTag::U8,
            ParsedType::U16 => TypeTag::U16,
            ParsedType::U32 => TypeTag::U32,
            ParsedType::U64 => TypeTag::U64,
            ParsedType::U128 => TypeTag::U128,
            ParsedType::U256 => TypeTag::U256,
            ParsedType::Bool => TypeTag::Bool,
            ParsedType::Address => TypeTag::Address,
            ParsedType::Signer => TypeTag::Signer,
            ParsedType::Vector(inner) => TypeTag::Vector(Box::new(inner.into_type_tag(mapping)?)),
            ParsedType::Struct(s) => TypeTag::Struct(Box::new(s.into_struct_tag(mapping)?)),
        })
    }
}

impl ParsedStructType {
    pub fn into_struct_tag(self, mapping: &impl Fn(&str) -> Option<AccountAddress>) -> Result<StructTag> {
        let Self { fq_name, type_args } = self;
        Ok(StructTag {
            address: fq_name.module.address.into_account_address(mapping)?,
            module: Identifier::new(fq_name.module.name)?,
            name: Identifier::new(fq_name.name)?,
            type_params: type_args
                .into_iter()
                .map(|t| t.into_type_tag(mapping))
                .collect::<Result<_>>()?,
        })
    }
}
