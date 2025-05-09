use crate::wallet::rebased::{IdentStr, error::Result};

use super::{
    annotated_value_min as A,
    binary_format_min::CompiledModule,
    language_storage_min::{ModuleId, StructTag, TypeTag},
    module_cache_min::GetModule,
};

/// The maximal value depth that we allow creating a layout for.
const MAX_VALUE_DEPTH: u64 = 128;

macro_rules! check_depth {
    ($depth:expr) => {
        if $depth > MAX_VALUE_DEPTH {
            /*
            anyhow::bail!("Exceeded max value recursion depth when creating struct layout")
            */
            return Err(crate::wallet::rebased::error::RebasedError::LayoutBuilderError(
                format!("Exceeded max value recursion depth when creating struct layout"),
            ));
        }
    };
}

pub enum TypeLayoutBuilder {}

impl TypeLayoutBuilder {
    /// Construct a WithTypes `TypeLayout` with fields from `t`.
    /// Panics if `resolver` cannot resolve a module whose types are referenced directly or
    /// transitively by `t`
    pub fn build_with_types(t: &TypeTag, resolver: &impl GetModule) -> Result<A::MoveTypeLayout> {
        Self::build(t, resolver, 0)
    }

    fn build(t: &TypeTag, resolver: &impl GetModule, depth: u64) -> Result<A::MoveTypeLayout> {
        use TypeTag::*;
        check_depth!(depth);
        Ok(match t {
            Bool => A::MoveTypeLayout::Bool,
            U8 => A::MoveTypeLayout::U8,
            U16 => A::MoveTypeLayout::U16,
            U32 => A::MoveTypeLayout::U32,
            U64 => A::MoveTypeLayout::U64,
            U128 => A::MoveTypeLayout::U128,
            U256 => A::MoveTypeLayout::U256,
            Address => A::MoveTypeLayout::Address,
            Signer => {
                return Err(crate::wallet::rebased::error::RebasedError::LayoutBuilderError(
                    format!("Type layouts cannot contain signer"),
                ));
            }
            Vector(elem_t) => A::MoveTypeLayout::Vector(Box::new(Self::build(elem_t, resolver, depth + 1)?)),
            Struct(s) => DatatypeLayoutBuilder::build(s, resolver, depth + 1)?.into_layout(),
        })
    }
}

pub enum DatatypeLayoutBuilder {}

impl DatatypeLayoutBuilder {
    /// Construct an expanded `TypeLayout` from `s`.
    /// Panics if `resolver` cannot resolved a module whose types are referenced directly or
    /// transitively by `s`.
    fn build(s: &StructTag, resolver: &impl GetModule, depth: u64) -> Result<A::MoveDatatypeLayout> {
        check_depth!(depth);
        let type_arguments = s
            .type_params
            .iter()
            .map(|t| TypeLayoutBuilder::build(t, resolver, depth))
            .collect::<Result<Vec<A::MoveTypeLayout>>>()?;
        Self::build_from_name(&s.module_id(), &s.name, type_arguments, resolver, depth)
    }

    fn build_from_name(
        declaring_module: &ModuleId,
        name: &IdentStr,
        type_arguments: Vec<A::MoveTypeLayout>,
        resolver: &impl GetModule,
        depth: u64,
    ) -> Result<A::MoveDatatypeLayout> {
        check_depth!(depth);
        let module = match resolver.get_module_by_id(declaring_module) {
            Err(_) | Ok(None) => Err(crate::wallet::rebased::error::RebasedError::LayoutBuilderError(
                format!("Could not find module"),
            ))?,
            Ok(Some(m)) => m,
        };
        match (
            module.borrow().find_struct_def_by_name(name.as_str()),
            module.borrow().find_enum_def_by_name(name.as_str()),
        ) {
            (Some((_, struct_def)), None) => Ok(A::MoveDatatypeLayout::Struct(Box::new(
                Self::build_from_struct_definition(module.borrow(), struct_def, type_arguments, resolver, depth)?,
            ))),
            (None, Some((_, enum_def))) => Ok(A::MoveDatatypeLayout::Enum(Box::new(Self::build_from_enum_definition(
                module.borrow(),
                enum_def,
                type_arguments,
                resolver,
                depth,
            )?))),
            (Some(_), Some(_)) => {
                return crate::wallet::rebased::error::RebasedError::LayoutBuilderError(format!(
                    "Found both struct and enum with name {}",
                    name
                ));
            }
            (None, None) => {
                return crate::wallet::rebased::error::RebasedError::LayoutBuilderError(format!(
                    "Could not find struct/enum named {0} in module {1}",
                    name.to_string(),
                    module.borrow().name()
                ));
            }
        }
    }

    fn build_from_struct_definition(
        m: &CompiledModule,
        s: &StructDefinition,
        type_arguments: Vec<A::MoveTypeLayout>,
        resolver: &impl GetModule,
        depth: u64,
    ) -> Result<A::MoveStructLayout> {
        check_depth!(depth);
        let s_handle = m.datatype_handle_at(s.struct_handle);
        if s_handle.type_parameters.len() != type_arguments.len() {
            return crate::wallet::rebased::error::RebasedError::LayoutBuilderError(format!(
                "Wrong number of type arguments for struct"
            ));
        }
        match &s.field_information {
            StructFieldInformation::Native => {
                return crate::wallet::rebased::error::RebasedError::LayoutBuilderError(format!(
                    "Can't extract fields for native struct"
                ));
            }
            StructFieldInformation::Declared(fields) => {
                let layouts = fields
                    .iter()
                    .map(|f| {
                        TypeLayoutBuilder::build_from_signature_token(
                            m,
                            &f.signature.0,
                            &type_arguments,
                            resolver,
                            depth,
                        )
                    })
                    .collect::<Result<Vec<A::MoveTypeLayout>>>()?;

                let mid = m.self_id();
                let type_params: Vec<TypeTag> = type_arguments.iter().map(|t| t.into()).collect();
                let type_ = StructTag {
                    address: *mid.address(),
                    module: mid.name().to_owned(),
                    name: m.identifier_at(s_handle.name).to_owned(),
                    type_params,
                };
                let fields = fields
                    .iter()
                    .map(|f| m.identifier_at(f.name).to_owned())
                    .zip(layouts)
                    .map(|(name, layout)| A::MoveFieldLayout::new(name, layout))
                    .collect();
                Ok(A::MoveStructLayout { type_, fields })
            }
        }
    }
}
