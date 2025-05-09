/// A `CompiledModule` defines the structure of a module which is the unit of
/// published code.
///
/// A `CompiledModule` contains a definition of types (with their fields) and
/// functions. It is a unit of code that can be used by transactions or other
/// modules.
///
/// A module is published as a single entry and it is retrieved as a single
/// blob.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
pub struct CompiledModule {
    /// Version number found during deserialization
    pub version: u32,
    /// Handle to self.
    pub self_module_handle_idx: ModuleHandleIndex,
    /// Handles to external dependency modules and self.
    pub module_handles: Vec<ModuleHandle>,
    /// Handles to external and internal types.
    pub datatype_handles: Vec<DatatypeHandle>,
    /// Handles to external and internal functions.
    pub function_handles: Vec<FunctionHandle>,
    /// Handles to fields.
    pub field_handles: Vec<FieldHandle>,
    /// Friend declarations, represented as a collection of handles to external
    /// friend modules.
    pub friend_decls: Vec<ModuleHandle>,

    /// Struct instantiations.
    pub struct_def_instantiations: Vec<StructDefInstantiation>,
    /// Function instantiations.
    pub function_instantiations: Vec<FunctionInstantiation>,
    /// Field instantiations.
    pub field_instantiations: Vec<FieldInstantiation>,

    /// Locals signature pool. The signature for all locals of the functions
    /// defined in the module.
    pub signatures: SignaturePool,

    /// All identifiers used in this module.
    pub identifiers: IdentifierPool,
    /// All address identifiers used in this module.
    pub address_identifiers: AddressIdentifierPool,
    /// Constant pool. The constant values used in the module.
    pub constant_pool: ConstantPool,

    pub metadata: Vec<Metadata>,

    /// Struct types defined in this module.
    pub struct_defs: Vec<StructDefinition>,
    /// Function defined in this module.
    pub function_defs: Vec<FunctionDefinition>,

    /// Enum types defined in this module.
    pub enum_defs: Vec<EnumDefinition>,
    /// Enum instantiations.
    pub enum_def_instantiations: Vec<EnumDefInstantiation>,
    // Enum packs
    pub variant_handles: Vec<VariantHandle>,
    // Enum pack instantiations
    pub variant_instantiation_handles: Vec<VariantInstantiationHandle>,
}
