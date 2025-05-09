use std::fmt::{self, Display};

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
