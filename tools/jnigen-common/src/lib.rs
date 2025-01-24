use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, Token};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnType {
    Unit,
    String,
    F64,
    F32,
    I32,
    I64,
    Bool,
    VecU8,
    ResultStringError(Box<ReturnType>),
    OptionString,
}

impl ReturnType {
    /// Get the rust type that matches the JNI interface.
    pub fn to_jni_type(&self) -> syn::ReturnType {
        match self {
            Self::Unit => syn::ReturnType::Default,
            Self::String => syn::parse_quote!(-> JString<'local>),
            Self::OptionString => syn::parse_quote!(-> JString<'local>),
            Self::F64 => syn::parse_quote!(-> jdouble),
            Self::F32 => syn::parse_quote!(-> jfloat ),
            Self::I32 => syn::parse_quote!(-> jint ),
            Self::I64 => syn::parse_quote!(-> jlong ),
            Self::Bool => syn::parse_quote!( -> jboolean ),
            Self::VecU8 => syn::parse_quote! { -> JByteArray<'local>},
            Self::ResultStringError(inner) => inner.to_jni_type(),
        }
    }

    /// Get the corresponding rust type.
    pub fn to_rust_type(&self) -> TokenStream {
        match self {
            Self::Unit => quote!(()),
            Self::String => quote!(String),
            Self::OptionString => quote!(Option<String>),
            Self::F64 => quote!(f64),
            Self::F32 => quote!(f32),
            Self::I32 => quote!(i32),
            Self::I64 => quote!(i64),
            Self::Bool => quote!(bool),
            Self::VecU8 => quote!(Vec<u8>),
            Self::ResultStringError(inner) => {
                let inner_type = inner.to_rust_type();
                quote!(Result<#inner_type, String>)
            }
        }
    }
    /// Wheteher or not this return value has a real return value in java (eg. is a void type or
    /// not)
    pub fn has_java_return_value(&self) -> bool {
        match self {
            Self::Unit => false,
            Self::ResultStringError(inner) => inner.has_java_return_value(),
            _ => true,
        }
    }

    /// Get the default value returned when a panic or an error occurs (will be ignored anyways since we throw an Exception in these cases)
    pub fn to_rust_panic_value(&self) -> TokenStream {
        match self {
            Self::Unit => quote!(()),
            Self::String => quote!(String::new()),
            Self::OptionString => quote!(None),
            Self::F64 | Self::F32 => quote!(0.0),
            Self::I32 | Self::I64 => quote!(0),
            Self::Bool => quote!(false),
            Self::VecU8 => quote!(Vec::default()),
            Self::ResultStringError(inner) => {
                let inner_panic_value = inner.to_rust_panic_value();
                let inner_type = inner.to_rust_type();
                quote!(Ok::<#inner_type, String>(#inner_panic_value))
            }
        }
    }

    /// Get the corresponding java type identifier.
    pub fn to_java_type(&self) -> &'static str {
        match self {
            Self::Unit => "void",
            Self::String | Self::OptionString => "String",
            Self::F64 => "double",
            Self::F32 => "float",
            Self::I32 => "int",
            Self::I64 => "long",
            Self::Bool => "boolean",
            Self::VecU8 => "byte[]",
            Self::ResultStringError(inner) => inner.to_java_type(),
        }
    }

    /// Construct the postlude needed to return this type from the rust JNI function.
    pub fn fn_postlude(&self, identity: &syn::Ident) -> TokenStream {
        match self {
            Self::Unit => quote!(),
            Self::String => quote! {
                // if an exception is being thrown, we cannot create a new string (returns JavaException Err type),
                // so just return a null object
                env.new_string(#identity).unwrap_or_else(|_| JString::from(JObject::null()))
            },
            Self::OptionString => {
                let string_postlude = Self::String.fn_postlude(identity);
                quote! {
                    match #identity {
                        None => JString::from(JObject::null()),
                        Some(#identity) => {
                            #string_postlude
                        }
                    }
                }
            }
            Self::Bool => quote! {
                match #identity {
                    true => 1,
                    false => 0,
                }
            },
            Self::VecU8 => {
                quote! (
                    let Ok(array) = env.new_byte_array(#identity.len() as jint) else {
                        return JByteArray::from(JObject::null());
                    };
                    let slice = #identity.as_ref();
                    // SAFETY: convert from &[u8] to &[i8] which is safe since they are
                    // both 1-byte objects.
                    let slice = unsafe {&*(slice as *const [u8] as  * const [i8])};
                    if env.set_byte_array_region(&array, 0, slice).is_err() {
                        return JByteArray::from(JObject::null());
                    }
                    array
                )
            }
            Self::ResultStringError(inner) => {
                let inner_postlude = inner.fn_postlude(identity);
                let inner_default = inner.to_rust_panic_value();
                let inner_type = inner.to_rust_type();
                quote!(
                    let #identity: #inner_type = #identity.unwrap_or_else(|e: String| {
                        env.throw(e).unwrap();
                        #inner_default
                    });
                    #inner_postlude
                )
            }
            // for all other types, we just return the variable containing the result
            _ => quote!(#identity),
        }
    }

    /// Try to convert a [`syn::ReturnType`] into a [`ReturnType`]
    pub fn parse(return_type: &syn::ReturnType) -> syn::Result<Self> {
        match return_type {
            syn::ReturnType::Default => Ok(Self::Unit),
            syn::ReturnType::Type(_, ty) => Self::parse_type(ty),
        }
    }

    pub fn parse_type(ty: &syn::Type) -> syn::Result<Self> {
        match ty {
            syn::Type::Path(path) => {
                if let Some(ident) = path.path.get_ident() {
                    match ident.to_string().as_str() {
                        "String" => Ok(Self::String),
                        "f64" => Ok(Self::F64),
                        "f32" => Ok(Self::F32),
                        "i64" => Ok(Self::I64),
                        "i32" => Ok(Self::I32),
                        "bool" => Ok(Self::Bool),
                        _ => Err(syn::Error::new(
                            ident.span(),
                            format!("unsupported return type: {}", ident),
                        )),
                    }
                } else {
                    // parse as Vec, result type or option type
                    if let Ok(rt) = syn::parse2::<VecArgument>(path.to_token_stream()) {
                        match rt.inner_type.to_token_stream().to_string().as_str() {
                            "u8" => Ok(Self::VecU8),
                            _ => Err(syn::Error::new(
                                rt.inner_type.span(),
                                "Vec inner type needs to be u8".to_string(),
                            )),
                        }
                    } else if let Ok(rt) = syn::parse2::<ResultType>(path.to_token_stream()) {
                        // make sure the return type is a supported type
                        let ok_type = Self::parse_type(&rt.ok_type)?;

                        // and that the error type is a String
                        let err_type = Self::parse_type(&rt.err_type)?;

                        if err_type != Self::String {
                            Err(syn::Error::new(
                                rt.err_type.span(),
                                "Result error type needs to be String",
                            ))
                        } else {
                            Ok(Self::ResultStringError(Box::new(ok_type)))
                        }
                    } else if let Ok(rt) = syn::parse2::<OptionArgument>(path.to_token_stream()) {
                        // make sure the inner type is a String
                        let inner_type = Self::parse_type(&rt.inner_type)?;
                        if inner_type != Self::String {
                            Err(syn::Error::new(
                                rt.inner_type.span(),
                                "Option inner type needs to be String",
                            ))
                        } else {
                            Ok(Self::OptionString)
                        }
                    } else {
                        Err(syn::Error::new(
                            path.span(),
                            "Return type needs to be Result<_, String> or Option<String>",
                        ))
                    }
                }
            }
            syn::Type::Tuple(tuple) => {
                // an empty tuple is a unit return type
                if tuple.elems.is_empty() {
                    Ok(Self::Unit)
                } else {
                    Err(syn::Error::new(
                        tuple.span(),
                        "only empty tuple return types (eg. unit type) is supported".to_string(),
                    ))
                }
            }

            unsupported => Err(syn::Error::new(
                unsupported.span(),
                "return type needs to be a pure identifier, no references and only owned values are supported"
                    .to_string(),
            )),
        }
    }
}

struct ResultType {
    _result_identity: syn::Ident,
    _opening_bracket: Token![<],
    ok_type: syn::Type,
    _separator: Token![,],
    err_type: syn::Type,
    _closing_bracket: Token![>],
}
impl Parse for ResultType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;

        if ident != "Result" {
            return Err(syn::Error::new(
                ident.span(),
                "return type needs to be Result".to_string(),
            ));
        }

        Ok(Self {
            _result_identity: ident,
            _opening_bracket: input.parse()?,
            ok_type: input.parse()?,
            _separator: input.parse()?,
            err_type: input.parse()?,
            _closing_bracket: input.parse()?,
        })
    }
}

/// The list of supported function argument types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentType {
    I32,
    I64,
    F64,
    F32,
    Bool,
    String,
    VecString,
    VecU8,
    Option(Box<Self>),
}

impl ArgumentType {
    /// Get the rust type that matches the JNI interface.
    pub fn to_jni_type(&self) -> syn::Type {
        match self {
            ArgumentType::I32 => syn::parse_quote! { jint },
            ArgumentType::I64 => syn::parse_quote! { jlong },
            Self::F64 => syn::parse_quote!(jdouble),
            Self::F32 => syn::parse_quote!(jfloat),
            Self::Bool => syn::parse_quote!(jboolean),
            ArgumentType::String => syn::parse_quote! { JString<'local> },
            ArgumentType::VecString => syn::parse_quote! { JObjectArray<'local>},
            ArgumentType::VecU8 => syn::parse_quote! { JByteArray<'local>},
            ArgumentType::Option(inner) => inner.to_jni_type(), //since all objects can be null in Java, everything basically has the same signature.
        }
    }

    /// Get the corresponding java type identifier.
    pub fn to_java_type(&self) -> &'static str {
        match self {
            Self::I32 => "int",
            Self::I64 => "long",
            Self::F32 => "float",
            Self::F64 => "double",
            Self::Bool => "boolean",
            Self::String => "String",
            Self::VecString => "String[]",
            Self::VecU8 => "byte[]",
            ArgumentType::Option(inner) => inner.to_java_type(),
        }
    }

    /// Construct the prelude needed to convert the passed JNI values into rust types.
    pub fn fn_prelude(&self, identity: &syn::Ident) -> TokenStream {
        match self {
            Self::String => quote! {
                let #identity: String = env.get_string(&#identity).expect("Could not get Java String").into();
            },
            Self::Bool => quote! {
                let #identity = #identity == 1;
            },
            Self::VecString => quote! {
                // here we need to iterate through all the elements and convert them into a rust Vec.
                // We use unwrap since we expect the array to actually be of the correct type etc
                // since we generate binding wrappers that have the correct type annotations
                // (eg. String[]) and since Java is a typed language we should not get anything
                // else passed in to our function.
                let #identity = {
                    let len = env.get_array_length(&#identity).expect("Could not get array length");

                    let mut rust_vec: Vec<String> = Vec::with_capacity(len as usize);
                    for i in 0..(len as usize) {
                        // get the object out of the array
                        let obj = env.get_object_array_element(&#identity, i as jsize).expect("Could not get object array item");

                        // and finally convert the object into a rust string
                        let string: String = env.get_string(&JString::from(obj)).expect("Could not get Java String").into();

                        rust_vec.push(string);
                    }
                    rust_vec
                };

            },
            Self::VecU8 => quote! {
                let #identity: Vec<u8> = env.convert_byte_array(&#identity).expect("Could not get Java byte array").into();
            },
            Self::Option(inner) => {
                let inner_prelude = inner.fn_prelude(identity);

                quote! {
                    let #identity = if #identity.is_null() {
                        None
                    } else {
                        #inner_prelude
                        Some(
                            #identity
                        )
                    };

                }
            }
            _ => quote!(),
        }
    }

    /// Try to convert a [`syn::FnArg`] into a [`ArgumentType`] together with the variable name.
    pub fn parse(arg: &syn::FnArg) -> syn::Result<(String, ArgumentType)> {
        let syn::FnArg::Typed(typed) = arg else {
            return Err(syn::Error::new(
                arg.span(),
                "only typed arguments are supported (no reciever arguments)",
            ));
        };

        // extract the argument identity name
        let argument_name = if let syn::Pat::Ident(ident) = &*typed.pat {
            ident.ident.to_string()
        } else {
            return Err(syn::Error::new(typed.span(), "type name needs to be an identifier"));
        };
        Ok((argument_name, parse_type(&typed.ty)?))
    }
}

fn parse_type(ty: &syn::Type) -> syn::Result<ArgumentType> {
    match &ty {
        syn::Type::Path(path) => {
            if let Some(ident) = path.path.get_ident() {
                match ident.to_string().as_str() {
                    "i32" => Ok(ArgumentType::I32),
                    "i64" => Ok(ArgumentType::I64),
                    "f32" => Ok(ArgumentType::F32),
                    "f64" => Ok(ArgumentType::F64),
                    "bool" => Ok(ArgumentType::Bool),
                    "String" => Ok(ArgumentType::String),
                    _ => Err(syn::Error::new(
                        ident.span(),
                        format!("unsupported argument type: {}", ident),
                    )),
                }
            } else {
                // try to parse the argument as a Vec<String>
                if let Ok(rt) = syn::parse2::<VecArgument>(path.to_token_stream()) {
                    // make sure that the type is String
                    match rt.inner_type.to_token_stream().to_string().as_str() {
                        "String" => Ok(ArgumentType::VecString),
                        "u8" => Ok(ArgumentType::VecU8),
                        _ => Err(syn::Error::new(
                            rt.inner_type.span(),
                            "Vec inner type needs to be String".to_string(),
                        )),
                    }
                } else if let Ok(rt) = syn::parse2::<OptionArgument>(path.to_token_stream()) {
                    // parse the inner type recursively
                    let inner = parse_type(&rt.inner_type)?;

                    // only types that are java objects can be null (i.e. not primitives)
                    match inner {
                        ArgumentType::VecU8 | ArgumentType::String | ArgumentType::VecString => {
                            Ok(ArgumentType::Option(Box::new(inner)))
                        }
                        _ => Err(syn::Error::new(
                            rt.inner_type.span(),
                            "Only object types are supported for Option<T>, not primitives.",
                        )),
                    }
                } else {
                    Err(syn::Error::new(
                        ty.span(),
                        "Only Vec<T> and Option<T> are supported except for primitives",
                    ))
                }
            }
        }
        unsupported => Err(syn::Error::new(
            unsupported.span(),
            "argument type needs to be a pure identifier, no references and only owned values are supported",
        )),
    }
}

struct VecArgument {
    _vec_identity: syn::Ident,
    _opening_bracket: Token![<],
    inner_type: syn::Type,
    _closing_bracket: Token![>],
}

impl Parse for VecArgument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;

        if ident != "Vec" {
            return Err(syn::Error::new(ident.span(), "not a Vec type".to_string()));
        }

        Ok(Self {
            _vec_identity: ident,
            _opening_bracket: input.parse()?,
            inner_type: input.parse()?,
            _closing_bracket: input.parse()?,
        })
    }
}
struct OptionArgument {
    _option_identity: syn::Ident,
    _opening_bracket: Token![<],
    inner_type: syn::Type,
    _closing_bracket: Token![>],
}

impl Parse for OptionArgument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: syn::Ident = input.parse()?;

        if ident != "Option" {
            return Err(syn::Error::new(ident.span(), "not an Option type".to_string()));
        }

        Ok(Self {
            _option_identity: ident,
            _opening_bracket: input.parse()?,
            inner_type: input.parse()?,
            _closing_bracket: input.parse()?,
        })
    }
}

// Parses the public name from an attribute that looks like:
//
//     #[public_name = "newName"]
//
// or returns `None` if the input is some other attribute.
pub fn get_public_name(attr: &syn::Attribute) -> syn::Result<Option<String>> {
    if !attr.path().is_ident("public_name") {
        return Ok(None);
    }

    if let syn::Meta::NameValue(meta) = &attr.meta {
        if let syn::Expr::Lit(expr) = &meta.value {
            if let syn::Lit::Str(lit_str) = &expr.lit {
                return Ok(Some(lit_str.value()));
            }
        }
    }

    let message = "expected #[public_name = \"...\"]";
    Err(syn::Error::new_spanned(attr, message))
}

#[cfg(test)]
mod tests {
    use crate::ReturnType;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn parse_result_ok() {
        let cases: Vec<(TokenStream, ReturnType)> = vec![
            (
                quote!(-> Result<(), String>),
                ReturnType::ResultStringError(Box::new(ReturnType::Unit)),
            ),
            (
                quote!(-> Result<String, String>),
                ReturnType::ResultStringError(Box::new(ReturnType::String)),
            ),
            (
                quote!(-> Result<f64, String>),
                ReturnType::ResultStringError(Box::new(ReturnType::F64)),
            ),
            (
                quote!(-> Result<i32, String>),
                ReturnType::ResultStringError(Box::new(ReturnType::I32)),
            ),
        ];

        for case in cases {
            let return_type: syn::ReturnType = syn::parse2(case.0).expect("should parse ok");
            let parsed = ReturnType::parse(&return_type).unwrap();
            assert_eq!(parsed, case.1);
        }
    }
    #[test]
    fn parse_result_not_string() {
        let return_type: syn::ReturnType = syn::parse_quote!(-> Result<(), Error>);
        let parsed = ReturnType::parse(&return_type);
        assert!(parsed.is_err());
    }

    #[test]
    fn parse_result_unsupported_ok_type() {
        let cases: Vec<TokenStream> = vec![
            quote!(-> Result<Option<f32>, String>),
            quote!(-> Result<Result<i32, Error>, String>),
            quote!(-> Result<usize, String>),
        ];

        for case in cases {
            let return_type: syn::ReturnType = syn::parse2(case).expect("should parse ok");
            let parsed = ReturnType::parse(&return_type);
            assert!(parsed.is_err());
        }
    }
}
