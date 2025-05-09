use log::info;
use std::sync::LazyLock;

use crate::wallet::rebased::{error::Result, mowe::annotated_value_min as A, runtime_value_min::MoveValue};

/// Visitor to deserialize annotated values or structs, bounding the size
/// budgeted for types and field names in the output. The visitor does not bound
/// the size of values, because they are assumed to already be bounded by
/// execution.
pub struct BoundedVisitor {
    /// Budget left to spend on field names and types.
    bound: usize,
}

impl BoundedVisitor {
    fn new(bound: usize) -> Self {
        Self { bound }
    }

    /// Deserialize `bytes` as a `MoveValue` with layout `layout`. Can fail if
    /// the bytes do not represent a value with this layout, or if the
    /// deserialized value exceeds the field/type size budget.
    pub fn deserialize_value(bytes: &[u8], layout: &A::MoveTypeLayout) -> Result<MoveValue> {
        let mut visitor = Self::default();
        MoveValue::visit_deserialize(bytes, layout, &mut visitor)
    }
}

impl Default for BoundedVisitor {
    fn default() -> Self {
        Self::new(*MAX_BOUND)
    }
}

/// Environment variable to override the default budget for deserialization.
/// This can be set at runtime to change the maximum size of values that can be
/// deserialized.
const MAX_BOUND_VAR_NAME: &str = "MAX_ANNOTATED_VALUE_SIZE";

/// Default budget for deserialization -- we're okay to spend an extra ~1MiB on
/// types and field information per value.
const DEFAULT_MAX_BOUND: usize = 1024 * 1024;

/// Budget for deserialization into an annotated Move value. This sets the
/// numbers of bytes that we are willing to spend on field names, type names
/// (etc) when deserializing a Move value into an annotated Move value.
///
/// Bounded deserialization is intended for use outside of the validator, and so
/// uses a fixed bound that needs to be set at startup rather than one that is
/// configured as part of the protocol.
///
/// If the environment variable `MAX_ANNOTATED_VALUE_SIZE` is unset we default
/// to `DEFAULT_MAX_BOUND` which allows ~1MiB additional space usage on types
/// and field information per value.
///
/// This is read only once and after that the value is cached. To change this
/// value you will need to restart the process with the new value set (or the
/// value unset if you wish to use the `DEFAULT_MAX_BOUND` value).
static MAX_BOUND: LazyLock<usize> = LazyLock::new(|| {
    let max_bound_opt = std::env::var(MAX_BOUND_VAR_NAME).ok().and_then(|s| s.parse().ok());
    if let Some(max_bound) = max_bound_opt {
        info!(
            "Using custom value for '{}' max bound: {}",
            MAX_BOUND_VAR_NAME, max_bound
        );
        max_bound
    } else {
        info!(
            "Using default value for '{}' -- max bound: {}",
            MAX_BOUND_VAR_NAME, DEFAULT_MAX_BOUND
        );
        DEFAULT_MAX_BOUND
    }
});
