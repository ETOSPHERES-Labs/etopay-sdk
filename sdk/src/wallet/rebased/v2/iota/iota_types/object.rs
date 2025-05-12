use serde_with::{Bytes, serde_as};
use std::{
    collections::BTreeMap,
    convert::TryFrom,
    fmt::{Debug, Display, Formatter},
    mem::size_of,
    sync::Arc,
};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::wallet::rebased::default_hash;

use super::{
    TransactionDigest,
    base_types::{IotaAddress, MoveObjectType, SequenceNumber},
    digests::ObjectDigest,
    move_package::MovePackage,
};

#[derive(Eq, PartialEq, Debug, Clone, Copy, Deserialize, Serialize, Hash, JsonSchema, Ord, PartialOrd)]
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

impl Display for Owner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AddressOwner(address) => {
                write!(f, "Account Address ( {} )", address)
            }
            Self::ObjectOwner(address) => {
                write!(f, "Object ID: ( {} )", address)
            }
            Self::Immutable => {
                write!(f, "Immutable")
            }
            Self::Shared { initial_shared_version } => {
                write!(f, "Shared( {} )", initial_shared_version.value())
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
#[serde(rename = "Object")]
pub struct ObjectInner {
    /// The meat of the object
    pub data: Data,
    /// The owner that unlocks this object
    pub owner: Owner,
    /// The digest of the transaction that created or last mutated this object
    pub previous_transaction: TransactionDigest,
    /// The amount of IOTA we would rebate if this object gets deleted.
    /// This number is re-calculated each time the object is mutated based on
    /// the present storage gas price.
    pub storage_rebate: u64,
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct Object(Arc<ObjectInner>);

impl From<ObjectInner> for Object {
    fn from(inner: ObjectInner) -> Self {
        Self(Arc::new(inner))
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub enum Data {
    /// An object whose governing logic lives in a published Move module
    Move(MoveObject),
    /// Map from each module name to raw serialized Move module bytes
    Package(MovePackage),
    // ... IOTA "native" types go here
}

#[serde_as]
#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize, Hash)]
pub struct MoveObject {
    /// The type of this object. Immutable
    pub(crate) type_: MoveObjectType,
    /// Number that increases each time a tx takes this object as a mutable
    /// input This is a lamport timestamp, not a sequentially increasing
    /// version
    pub(crate) version: SequenceNumber,
    /// BCS bytes of a Move struct value
    #[serde_as(as = "Bytes")]
    pub(crate) contents: Vec<u8>,
}

impl Object {
    pub fn into_inner(self) -> ObjectInner {
        match Arc::try_unwrap(self.0) {
            Ok(inner) => inner,
            Err(inner_arc) => (*inner_arc).clone(),
        }
    }

    pub fn as_inner(&self) -> &ObjectInner {
        &self.0
    }

    pub fn owner(&self) -> &Owner {
        &self.0.owner
    }

    pub fn new_from_genesis(data: Data, owner: Owner, previous_transaction: TransactionDigest) -> Self {
        ObjectInner {
            data,
            owner,
            previous_transaction,
            storage_rebate: 0,
        }
        .into()
    }

    /// Create a new Move object
    pub fn new_move(o: MoveObject, owner: Owner, previous_transaction: TransactionDigest) -> Self {
        ObjectInner {
            data: Data::Move(o),
            owner,
            previous_transaction,
            storage_rebate: 0,
        }
        .into()
    }

    pub fn new_package_from_data(data: Data, previous_transaction: TransactionDigest) -> Self {
        ObjectInner {
            data,
            owner: Owner::Immutable,
            previous_transaction,
            storage_rebate: 0,
        }
        .into()
    }

    // Note: this will panic if `modules` is empty
    pub fn new_from_package(package: MovePackage, previous_transaction: TransactionDigest) -> Self {
        Self::new_package_from_data(Data::Package(package), previous_transaction)
    }

    // pub fn new_package<'p>(
    //     modules: &[CompiledModule],
    //     previous_transaction: TransactionDigest,
    //     max_move_package_size: u64,
    //     move_binary_format_version: u32,
    //     dependencies: impl IntoIterator<Item = &'p MovePackage>,
    // ) -> Result<Self, ExecutionError> {
    //     Ok(Self::new_package_from_data(
    //         Data::Package(MovePackage::new_initial(
    //             modules,
    //             max_move_package_size,
    //             move_binary_format_version,
    //             dependencies,
    //         )?),
    //         previous_transaction,
    //     ))
    // }

    // pub fn new_upgraded_package<'p>(
    //     previous_package: &MovePackage,
    //     new_package_id: ObjectID,
    //     modules: &[CompiledModule],
    //     previous_transaction: TransactionDigest,
    //     protocol_config: &ProtocolConfig,
    //     dependencies: impl IntoIterator<Item = &'p MovePackage>,
    // ) -> Result<Self, ExecutionError> {
    //     Ok(Self::new_package_from_data(
    //         Data::Package(previous_package.new_upgraded(new_package_id, modules, protocol_config, dependencies)?),
    //         previous_transaction,
    //     ))
    // }

    // pub fn new_package_for_testing(
    //     modules: &[CompiledModule],
    //     previous_transaction: TransactionDigest,
    //     dependencies: impl IntoIterator<Item = MovePackage>,
    // ) -> Result<Self, ExecutionError> {
    //     let dependencies: Vec<_> = dependencies.into_iter().collect();
    //     let config = ProtocolConfig::get_for_max_version_UNSAFE();
    //     Self::new_package(
    //         modules,
    //         previous_transaction,
    //         config.max_move_package_size(),
    //         config.move_binary_format_version(),
    //         &dependencies,
    //     )
    // }

    // /// Create a system package which is not subject to size limits. Panics if
    // /// the object ID is not a known system package.
    // pub fn new_system_package(
    //     modules: &[CompiledModule],
    //     version: SequenceNumber,
    //     dependencies: Vec<ObjectID>,
    //     previous_transaction: TransactionDigest,
    // ) -> Self {
    //     let ret = Self::new_package_from_data(
    //         Data::Package(MovePackage::new_system(version, modules, dependencies)),
    //         previous_transaction,
    //     );

    //     #[cfg(not(msim))]
    //     assert!(ret.is_system_package());

    //     ret
    // }
}

impl std::ops::Deref for Object {
    type Target = ObjectInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Object {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Arc::make_mut(&mut self.0)
    }
}

impl ObjectInner {
    /// Returns true if the object is a system package.
    // pub fn is_system_package(&self) -> bool {
    //     self.is_package() && is_system_package(self.id())
    // }

    // pub fn is_immutable(&self) -> bool {
    //     self.owner.is_immutable()
    // }

    // pub fn is_address_owned(&self) -> bool {
    //     self.owner.is_address_owned()
    // }

    // pub fn is_child_object(&self) -> bool {
    //     self.owner.is_child_object()
    // }

    // pub fn is_shared(&self) -> bool {
    //     self.owner.is_shared()
    // }

    // pub fn get_single_owner(&self) -> Option<IotaAddress> {
    //     self.owner.get_owner_address().ok()
    // }

    // // It's a common pattern to retrieve both the owner and object ID
    // // together, if it's owned by a singler owner.
    // pub fn get_owner_and_id(&self) -> Option<(Owner, ObjectID)> {
    //     Some((self.owner, self.id()))
    // }

    // /// Return true if this object is a Move package, false if it is a Move
    // /// value
    // pub fn is_package(&self) -> bool {
    //     matches!(&self.data, Data::Package(_))
    // }

    // pub fn compute_object_reference(&self) -> ObjectRef {
    //     (self.id(), self.version(), self.digest())
    // }

    pub fn digest(&self) -> ObjectDigest {
        ObjectDigest::new(default_hash(self))
    }

    // pub fn id(&self) -> ObjectID {
    //     use Data::*;

    //     match &self.data {
    //         Move(v) => v.id(),
    //         Package(m) => m.id(),
    //     }
    // }

    pub fn version(&self) -> SequenceNumber {
        use Data::*;

        match &self.data {
            Move(o) => o.version(),
            Package(p) => p.version(),
        }
    }
}
