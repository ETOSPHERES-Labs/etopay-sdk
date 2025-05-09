use std::fmt;

use alloy_primitives::U256;

use crate::wallet::rebased::error::Result;
use crate::wallet::rebased::{RebasedError, v2::mowe::move_core_types::AccountAddress};

use super::{NumberFormat, parse_address_number};

// Parsed Address, either a name or a numerical address
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum ParsedAddress {
    Named(String),
    Numerical(NumericalAddress),
}

/// Numerical address represents non-named address values
/// or the assigned value of a named address
#[derive(Clone, Copy)]
pub struct NumericalAddress {
    /// the number for the address
    bytes: AccountAddress,
    /// The format (e.g. decimal or hex) for displaying the number
    format: NumberFormat,
}

impl fmt::Debug for NumericalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl AsRef<[u8]> for NumericalAddress {
    fn as_ref(&self) -> &[u8] {
        self.bytes.as_ref()
    }
}

impl fmt::Display for NumericalAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.format {
            NumberFormat::Decimal => {
                let n = U256::from_be_bytes(*self.bytes.to_bytes());
                write!(f, "{}", n)
            }
            NumberFormat::Hex => write!(f, "{:#X}", self),
        }
    }
}

impl fmt::UpperHex for NumericalAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let encoded = hex::encode_upper(self.as_ref());
        let dropped = encoded.chars().skip_while(|c| c == &'0').collect::<String>();
        let prefix = if f.alternate() { "0x" } else { "" };
        if dropped.is_empty() {
            write!(f, "{}0", prefix)
        } else {
            write!(f, "{}{}", prefix, dropped)
        }
    }
}

impl PartialEq for NumericalAddress {
    fn eq(&self, other: &Self) -> bool {
        let Self {
            bytes: self_bytes,
            format: _,
        } = self;
        let Self {
            bytes: other_bytes,
            format: _,
        } = other;
        self_bytes == other_bytes
    }
}
impl Eq for NumericalAddress {}

impl PartialEq<AccountAddress> for NumericalAddress {
    fn eq(&self, other: &AccountAddress) -> bool {
        let Self {
            bytes: self_bytes,
            format: _,
        } = self;
        self_bytes == other
    }
}

impl NumericalAddress {
    pub fn parse_str(s: &str) -> Result<NumericalAddress> {
        match parse_address_number(s) {
            Some((n, format)) => Ok(NumericalAddress { bytes: n, format }),
            None =>
            // TODO the kind of error is in an unstable nightly API
            // But currently the only way this should fail is if the number is too long
            {
                Err(RebasedError::ParserError(format!(
                    "Invalid address literal. The numeric value is too large. \
                    The maximum size is {} bytes",
                    AccountAddress::LENGTH,
                )))
            }
        }
    }

    pub fn into_inner(self) -> AccountAddress {
        self.bytes
    }
}

impl ParsedAddress {
    pub fn into_account_address(self, mapping: &impl Fn(&str) -> Option<AccountAddress>) -> Result<AccountAddress> {
        match self {
            Self::Named(n) => {
                mapping(n.as_str()).ok_or_else(|| RebasedError::ParserError(format!("Unbound named address: '{}'", n)))
            }
            Self::Numerical(a) => Ok(a.into_inner()),
        }
    }
}
