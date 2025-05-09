use std::{borrow::Borrow, fmt::Debug};

use super::{binary_format_min::CompiledModule, language_storage_min::ModuleId};

/// A persistent storage that can fetch the bytecode for a given module id
/// TODO: do we want to implement this in a way that allows clients to cache
/// struct layouts?
pub trait GetModule {
    type Error: Debug;
    type Item: Borrow<CompiledModule>;

    fn get_module_by_id(&self, id: &ModuleId) -> Result<Option<Self::Item>, Self::Error>;
}
