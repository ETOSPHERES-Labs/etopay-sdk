use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::{Bytes, serde_as};
use std::collections::{BTreeMap, BTreeSet};

use crate::wallet::rebased::v2::iota::iota_types::base_types::ObjectID;
use crate::wallet::rebased::v2::iota::iota_types::base_types::SequenceNumber;

// serde_bytes::ByteBuf is an analog of Vec<u8> with built-in fast
// serialization.
#[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MovePackage {
    pub(crate) id: ObjectID,
    /// Most move packages are uniquely identified by their ID (i.e. there is
    /// only one version per ID), but the version is still stored because
    /// one package may be an upgrade of another (at a different ID), in
    /// which case its version will be one greater than the version of the
    /// upgraded package.
    ///
    /// Framework packages are an exception to this rule -- all versions of the
    /// framework packages exist at the same ID, at increasing versions.
    ///
    /// In all cases, packages are referred to by move calls using just their
    /// ID, and they are always loaded at their latest version.
    pub(crate) version: SequenceNumber,
    // TODO use session cache
    #[serde_as(as = "BTreeMap<_, Bytes>")]
    pub(crate) module_map: BTreeMap<String, Vec<u8>>,

    /// Maps struct/module to a package version where it was first defined,
    /// stored as a vector for simple serialization and deserialization.
    pub(crate) type_origin_table: Vec<TypeOrigin>,

    // For each dependency, maps original package ID to the info about the (upgraded) dependency
    // version that this package is using
    pub(crate) linkage_table: BTreeMap<ObjectID, UpgradeInfo>,
}

/// Identifies a struct and the module it was defined in
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize, Hash, JsonSchema)]
pub struct TypeOrigin {
    pub module_name: String,
    // `struct_name` alias to support backwards compatibility with the old name
    #[serde(alias = "struct_name")]
    pub datatype_name: String,
    pub package: ObjectID,
}

/// Upgraded package info for the linkage table
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash, JsonSchema)]
pub struct UpgradeInfo {
    /// ID of the upgraded packages
    pub upgraded_id: ObjectID,
    /// Version of the upgraded package
    pub upgraded_version: SequenceNumber,
}
