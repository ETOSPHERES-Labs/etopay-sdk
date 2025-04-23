use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use super::{super::serde::Readable, AccountAddress, HexAccountAddress};

#[serde_as]
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectID(#[serde_as(as = "Readable<HexAccountAddress, _>")] AccountAddress);

/// temporary implementation to ease impl
impl From<ObjectID> for iota_sdk_rebased::types::base_types::ObjectID {
    fn from(value: ObjectID) -> Self {
        Self::new(value.0.to_inner())
    }
}
