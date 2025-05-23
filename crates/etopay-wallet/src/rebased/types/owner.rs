use super::{IotaAddress, SequenceNumber};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize, Hash, Ord, PartialOrd)]
#[allow(clippy::enum_variant_names)]
pub enum Owner {
    /// Object is exclusively owned by a single address, and is mutable.
    AddressOwner(IotaAddress),
    /// Object is exclusively owned by a single object, and is mutable.
    /// The object ID is converted to IotaAddress as IotaAddress is universal.
    ObjectOwner(IotaAddress),
    /// Object is shared, can be used by any address, and is mutable.
    Shared {
        /// The version at which the object became shared
        initial_shared_version: SequenceNumber,
    },
    /// Object is immutable, and hence ownership doesn't matter.
    Immutable,
}
