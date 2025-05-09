use super::BRIDGE_ADDRESS;
use super::IOTA_FRAMEWORK_ADDRESS;
use super::IOTA_SYSTEM_ADDRESS;
use super::MOVE_STDLIB_ADDRESS;
use super::STARDUST_ADDRESS;

use crate::wallet::rebased::v2::mowe::move_core_types::AccountAddress;
use crate::wallet::rebased::v2::mowe::move_core_types::language_storage::StructTag;
use crate::wallet::rebased::{error::Result, v2::TypeTag};
/// Parse `s` as a type: Either a struct type (see `parse_iota_struct_tag`), a
/// primitive type, or a vector with a type parameter. Parsing succeeds if and
/// only if `s` matches this format exactly, with no remaining input. This
/// function is intended for use within the authority codebase.
pub fn parse_iota_type_tag(s: &str) -> Result<TypeTag> {
    //use move_core_types::parsing::types::ParsedType;
    use crate::wallet::rebased::v2::mowe::ParsedType;
    ParsedType::parse(s)?.into_type_tag(&resolve_address)
}
/// Resolve well-known named addresses into numeric addresses.
pub fn resolve_address(addr: &str) -> Option<AccountAddress> {
    match addr {
        "std" => Some(*MOVE_STDLIB_ADDRESS),
        "iota" => Some(*IOTA_FRAMEWORK_ADDRESS),
        "iota_system" => Some(*IOTA_SYSTEM_ADDRESS),
        "stardust" => Some(*STARDUST_ADDRESS),
        "bridge" => Some(*BRIDGE_ADDRESS),
        _ => None,
    }
}

/// Parse `s` as a struct type: A fully-qualified name, optionally followed by a
/// list of type parameters (types -- see `parse_iota_type_tag`, separated by
/// commas, surrounded by angle brackets). Parsing succeeds if and only if `s`
/// matches this format exactly, with no remaining input. This function is
/// intended for use within the authority codebase.
pub fn parse_iota_struct_tag(s: &str) -> Result<StructTag> {
    //use move_core_types::parsing::types::ParsedStructType;
    use crate::wallet::rebased::v2::mowe::move_core_types::parsing::types::ParsedStructType;
    ParsedStructType::parse(s)?.into_struct_tag(&resolve_address)
}
