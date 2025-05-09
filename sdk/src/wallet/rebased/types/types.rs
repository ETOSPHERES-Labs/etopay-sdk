use std::fmt::{self, Display};

use digest::consts::U256;

use crate::wallet::rebased::{MoveStruct, MoveValue};

use super::{AccountAddress, ParsedAddress, Parser};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TypeToken {
    Whitespace,
    Ident,
    AddressIdent,
    ColonColon,
    Lt,
    Gt,
    Comma,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ValueToken {
    Number,
    NumberTyped,
    True,
    False,
    ByteString,
    HexString,
    Utf8String,
    Ident,
    AtSign,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Colon,
    ColonColon,
    Whitespace,
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParsedValue<Extra: ParsableValue = ()> {
    Address(ParsedAddress),
    InferredNum(U256),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    U256(U256),
    Bool(bool),
    Vector(Vec<ParsedValue<Extra>>),
    Struct(Vec<ParsedValue<Extra>>),
    Custom(Extra),
}

pub trait ParsableValue: Sized + Send + Sync + Clone + 'static {
    type ConcreteValue: Send;
    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        parser: &mut Parser<'a, ValueToken, I>,
    ) -> Option<anyhow::Result<Self>>;

    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue>;
    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue>;
    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue>;
    fn into_concrete_value(
        self,
        mapping: &impl Fn(&str) -> Option<AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue>;
}

impl ParsableValue for () {
    type ConcreteValue = MoveValue;
    fn parse_value<'a, I: Iterator<Item = (ValueToken, &'a str)>>(
        _: &mut Parser<'a, ValueToken, I>,
    ) -> Option<anyhow::Result<Self>> {
        None
    }
    fn move_value_into_concrete(v: MoveValue) -> anyhow::Result<Self::ConcreteValue> {
        Ok(v)
    }

    fn concrete_vector(elems: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MoveValue::Vector(elems))
    }

    fn concrete_struct(values: Vec<Self::ConcreteValue>) -> anyhow::Result<Self::ConcreteValue> {
        Ok(MoveValue::Struct(MoveStruct(values)))
    }
    fn into_concrete_value(
        self,
        _mapping: &impl Fn(&str) -> Option<AccountAddress>,
    ) -> anyhow::Result<Self::ConcreteValue> {
        unreachable!()
    }
}

impl Display for ValueToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let s = match self {
            ValueToken::Number => "[num]",
            ValueToken::NumberTyped => "[num typed]",
            ValueToken::True => "true",
            ValueToken::False => "false",
            ValueToken::ByteString => "[byte string]",
            ValueToken::Utf8String => "[utf8 string]",
            ValueToken::HexString => "[hex string]",
            ValueToken::Whitespace => "[whitespace]",
            ValueToken::Ident => "[identifier]",
            ValueToken::AtSign => "@",
            ValueToken::LBrace => "{",
            ValueToken::RBrace => "}",
            ValueToken::LBracket => "[",
            ValueToken::RBracket => "]",
            ValueToken::LParen => "(",
            ValueToken::RParen => ")",
            ValueToken::Comma => ",",
            ValueToken::Colon => ":",
            ValueToken::ColonColon => "::",
        };
        fmt::Display::fmt(s, formatter)
    }
}
