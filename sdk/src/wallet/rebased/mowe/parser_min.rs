use alloy_primitives::U256;

use crate::wallet::rebased::{AccountAddress, error::Result};

use super::language_storage_min::StructTag;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Clone, Copy)]
#[repr(u32)]
/// Number format enum, the u32 value represents the base
pub enum NumberFormat {
    Decimal = 10,
    Hex = 16,
}

const U256_MAX_DECIMAL_DIGITS: usize = 241 * AccountAddress::LENGTH / 100 + 1;

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

/// Parse `s` as a struct type: A fully-qualified name, optionally followed by a
/// list of type parameters (types -- see `parse_iota_type_tag`, separated by
/// commas, surrounded by angle brackets). Parsing succeeds if and only if `s`
/// matches this format exactly, with no remaining input. This function is
/// intended for use within the authority codebase.
pub fn parse_iota_struct_tag(s: &str) -> Result<StructTag> {
    use super::types_min::ParsedStructType;
    ParsedStructType::parse(s)?.into_struct_tag(&resolve_address)
}

static MOVE_STDLIB_ADDRESS: &str = "0x1";
static IOTA_FRAMEWORK_ADDRESS: &str = "0x2";
static IOTA_SYSTEM_ADDRESS: &str = "0x3";
static IOTA_CLOCK_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000006"; // normalizeIotaObjectId('0x6');
static IOTA_SYSTEM_MODULE_NAME: &str = "iota_system";
static IOTA_TYPE_ARG: &str = "0x2::iota::IOTA";
static IOTA_SYSTEM_STATE_OBJECT_ID: &str = "0x0000000000000000000000000000000000000000000000000000000000000005"; // normalizeIotaObjectId('0x5');

static STARDUST_ADDRESS: &str = "0x000000000000000000000000000000000000000000000000000000000000107a"; // STARDUST_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs
static BRIDGE_ADDRESS: &str = "0x000000000000000000000000000000000000000000000000000000000000000b"; // BRIDGE_ADDRESS, - don't know if it's valid, took it from crates/iota-types/src/lib.rs

/// Resolve well-known named addresses into numeric addresses.
pub fn resolve_address(addr: &str) -> Option<AccountAddress> {
    match addr {
        "std" => Some(AccountAddress::from_hex_literal(MOVE_STDLIB_ADDRESS).unwrap()),
        "iota" => Some(AccountAddress::from_hex_literal(IOTA_FRAMEWORK_ADDRESS).unwrap()),
        "iota_system" => Some(AccountAddress::from_hex_literal(IOTA_SYSTEM_ADDRESS).unwrap()),
        "stardust" => Some(AccountAddress::from_hex_literal(STARDUST_ADDRESS).unwrap()),
        "bridge" => Some(AccountAddress::from_hex_literal(BRIDGE_ADDRESS).unwrap()),
        _ => None,
    }
}
