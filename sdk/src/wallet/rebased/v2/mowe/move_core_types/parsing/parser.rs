use crate::wallet::rebased::error::RebasedError;
use crate::wallet::rebased::error::Result;

use super::NumericalAddress;
use super::ParsedAddress;
use super::ParsedFqName;
use super::ParsedModuleId;
use super::ParsedStructType;
use super::ParsedType;
use super::TypeToken;
use super::ValueToken;
use crate::wallet::rebased::v2::mowe::move_core_types::AccountAddress;

use alloy_primitives::U256;

use std::{fmt::Display, iter::Peekable, num::ParseIntError};

const MAX_TYPE_DEPTH: u64 = 128;
const MAX_TYPE_NODE_COUNT: u64 = 256;
// See: https://stackoverflow.com/questions/43787672/the-max-number-of-digits-in-an-int-based-on-number-of-bits
const U256_MAX_DECIMAL_DIGITS: usize = 241 * AccountAddress::LENGTH / 100 + 1;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u32)]
/// Number format enum, the u32 value represents the base
pub enum NumberFormat {
    Decimal = 10,
    Hex = 16,
}

impl ParsedType {
    pub fn parse(s: &str) -> Result<ParsedType> {
        parse(s, |parser| parser.parse_type())
    }
}

impl ParsedStructType {
    pub fn parse(s: &str) -> Result<ParsedStructType> {
        let ty = parse(s, |parser| parser.parse_type())
            .map_err(|e| RebasedError::ParserError(format!("Invalid struct type: {}. Got error: {}", s, e)))?;
        match ty {
            ParsedType::Struct(s) => Ok(s),
            _ => return Err(RebasedError::ParserError(format!("Invalid struct type: {}", s))),
        }
    }
}

pub trait Token: Display + Copy + Eq {
    fn is_whitespace(&self) -> bool;
    fn next_token(s: &str) -> Result<Option<(Self, usize)>>;
    fn tokenize(mut s: &str) -> Result<Vec<(Self, &str)>> {
        let mut v = vec![];
        while let Some((tok, n)) = Self::next_token(s)? {
            v.push((tok, &s[..n]));
            s = &s[n..];
        }
        Ok(v)
    }
}

pub(crate) fn parse<'a, Tok: Token, R>(
    s: &'a str,
    f: impl FnOnce(&mut Parser<'a, Tok, std::vec::IntoIter<(Tok, &'a str)>>) -> Result<R>,
) -> Result<R> {
    let tokens: Vec<_> = Tok::tokenize(s)?
        .into_iter()
        .filter(|(tok, _)| !tok.is_whitespace())
        .collect();
    let mut parser = Parser::new(tokens);
    let res = f(&mut parser)?;
    if let Ok((_, contents)) = parser.advance_any() {
        return Err(RebasedError::ParserError(format!(
            "Expected end of token stream. Got: {}",
            contents
        )));
    }
    Ok(res)
}

pub struct Parser<'a, Tok: Token, I: Iterator<Item = (Tok, &'a str)>> {
    count: u64,
    it: Peekable<I>,
}

impl ParsedModuleId {
    pub fn parse(s: &str) -> Result<ParsedModuleId> {
        parse(s, |parser| parser.parse_module_id())
    }
}

impl<'a, I: Iterator<Item = (TypeToken, &'a str)>> Parser<'a, TypeToken, I> {
    pub fn parse_module_id(&mut self) -> Result<ParsedModuleId> {
        let (tok, contents) = self.advance_any()?;
        self.parse_module_id_impl(tok, contents)
    }

    pub fn parse_fq_name(&mut self) -> Result<ParsedFqName> {
        let (tok, contents) = self.advance_any()?;
        self.parse_fq_name_impl(tok, contents)
    }

    pub fn parse_type(&mut self) -> Result<ParsedType> {
        self.parse_type_impl(0)
    }

    pub fn parse_module_id_impl(&mut self, tok: TypeToken, contents: &'a str) -> Result<ParsedModuleId> {
        let tok = match tok {
            TypeToken::Ident => ValueToken::Ident,
            TypeToken::AddressIdent => ValueToken::Number,
            tok => {
                return Err(RebasedError::ParserError(format!(
                    "unexpected token {tok}, expected address"
                )));
            }
        };
        let address = parse_address_impl(tok, contents)?;
        self.advance(TypeToken::ColonColon)?;
        let name = self.advance(TypeToken::Ident)?.to_owned();
        Ok(ParsedModuleId { address, name })
    }

    pub fn parse_fq_name_impl(&mut self, tok: TypeToken, contents: &'a str) -> Result<ParsedFqName> {
        let module = self.parse_module_id_impl(tok, contents)?;
        self.advance(TypeToken::ColonColon)?;
        let name = self.advance(TypeToken::Ident)?.to_owned();
        Ok(ParsedFqName { module, name })
    }

    fn parse_type_impl(&mut self, depth: u64) -> Result<ParsedType> {
        self.count += 1;

        if depth > MAX_TYPE_DEPTH || self.count > MAX_TYPE_NODE_COUNT {
            return Err(RebasedError::ParserError(format!(
                "Type exceeds maximum nesting depth or node count"
            )));
        }

        Ok(match self.advance_any()? {
            (TypeToken::Ident, "u8") => ParsedType::U8,
            (TypeToken::Ident, "u16") => ParsedType::U16,
            (TypeToken::Ident, "u32") => ParsedType::U32,
            (TypeToken::Ident, "u64") => ParsedType::U64,
            (TypeToken::Ident, "u128") => ParsedType::U128,
            (TypeToken::Ident, "u256") => ParsedType::U256,
            (TypeToken::Ident, "bool") => ParsedType::Bool,
            (TypeToken::Ident, "address") => ParsedType::Address,
            (TypeToken::Ident, "signer") => ParsedType::Signer,
            (TypeToken::Ident, "vector") => {
                self.advance(TypeToken::Lt)?;
                let ty = self.parse_type_impl(depth + 1)?;
                self.advance(TypeToken::Gt)?;
                ParsedType::Vector(Box::new(ty))
            }

            (tok @ (TypeToken::Ident | TypeToken::AddressIdent), contents) => {
                let fq_name = self.parse_fq_name_impl(tok, contents)?;
                let type_args = match self.peek_tok() {
                    Some(TypeToken::Lt) => {
                        self.advance(TypeToken::Lt)?;
                        let type_args = self.parse_list(
                            |parser| parser.parse_type_impl(depth + 1),
                            TypeToken::Comma,
                            TypeToken::Gt,
                            true,
                        )?;
                        self.advance(TypeToken::Gt)?;
                        if type_args.is_empty() {
                            return Err(RebasedError::ParserError(format!(
                                "expected at least one type argument"
                            )));
                        }
                        type_args
                    }
                    _ => vec![],
                };
                ParsedType::Struct(ParsedStructType { fq_name, type_args })
            }
            (tok, _) => {
                return Err(RebasedError::ParserError(format!(
                    "unexpected token {tok}, expected type"
                )));
            }
        })
    }
}

impl<'a, Tok: Token, I: Iterator<Item = (Tok, &'a str)>> Parser<'a, Tok, I> {
    pub fn new<T: IntoIterator<Item = (Tok, &'a str), IntoIter = I>>(v: T) -> Self {
        Self {
            count: 0,
            it: v.into_iter().peekable(),
        }
    }

    pub fn advance_any(&mut self) -> Result<(Tok, &'a str)> {
        match self.it.next() {
            Some(tok) => Ok(tok),
            None => return Err(RebasedError::ParserError(format!("unexpected end of tokens"))),
        }
    }

    pub fn advance(&mut self, expected_token: Tok) -> Result<&'a str> {
        let (t, contents) = self.advance_any()?;
        if t != expected_token {
            return Err(RebasedError::ParserError(format!(
                "expected token {}, got {}",
                expected_token, t
            )));
        }
        Ok(contents)
    }

    pub fn peek(&mut self) -> Option<(Tok, &'a str)> {
        self.it.peek().copied()
    }

    pub fn peek_tok(&mut self) -> Option<Tok> {
        self.it.peek().map(|(tok, _)| *tok)
    }

    pub fn parse_list<R>(
        &mut self,
        parse_list_item: impl Fn(&mut Self) -> Result<R>,
        delim: Tok,
        end_token: Tok,
        allow_trailing_delim: bool,
    ) -> Result<Vec<R>> {
        let is_end = |tok_opt: Option<Tok>| -> bool { tok_opt.map(|tok| tok == end_token).unwrap_or(true) };
        let mut v = vec![];
        while !is_end(self.peek_tok()) {
            v.push(parse_list_item(self)?);
            if is_end(self.peek_tok()) {
                break;
            }
            self.advance(delim)?;
            if is_end(self.peek_tok()) {
                if allow_trailing_delim {
                    break;
                } else {
                    return Err(RebasedError::ParserError(format!(
                        "Invalid type list: trailing delimiter '{}'",
                        delim
                    )));
                }
            }
        }
        Ok(v)
    }
}

pub fn parse_address_impl(tok: ValueToken, contents: &str) -> Result<ParsedAddress> {
    Ok(match tok {
        ValueToken::Number => ParsedAddress::Numerical(NumericalAddress::parse_str(contents).map_err(|s| {
            RebasedError::ParserError(format!(
                "Failed to parse numerical address '{}'. Got error: {}",
                contents, s
            ))
        })?),
        ValueToken::Ident => ParsedAddress::Named(contents.to_owned()),
        _ => {
            return Err(RebasedError::ParserError(format!(
                "unexpected token {}, expected identifier or number",
                tok
            )));
        }
    })
}

// Parse an address from a decimal or hex encoding
pub fn parse_address_number(s: &str) -> Option<(AccountAddress, NumberFormat)> {
    let (txt, base) = determine_num_text_and_base(s);
    let txt = txt.replace('_', "");
    let max_len = match base {
        NumberFormat::Hex => AccountAddress::LENGTH * 2,
        NumberFormat::Decimal => U256_MAX_DECIMAL_DIGITS,
    };
    if txt.len() > max_len {
        return None;
    }
    let parsed = U256::from_str_radix(
        &txt,
        match base {
            NumberFormat::Hex => 16,
            NumberFormat::Decimal => 10,
        },
    )
    .ok()?;
    Some((AccountAddress::new(parsed.to_be_bytes()), base))
}

// Determines the base of the number literal, depending on the prefix
pub(crate) fn determine_num_text_and_base(s: &str) -> (&str, NumberFormat) {
    match s.strip_prefix("0x") {
        Some(s_hex) => (s_hex, NumberFormat::Hex),
        None => (s, NumberFormat::Decimal),
    }
}
