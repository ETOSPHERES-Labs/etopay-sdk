use jnigen_common::ArgumentType;
use jnigen_common::ReturnType;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn generate(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    generate_inner(attr.into(), item.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
/// Constructs the prelude for the module, including use stataments and helper functions
fn mod_prelude() -> TokenStream {
    quote!(
        #![allow(clippy::unused_unit)]
        use jni::objects::{JClass, JString, JObject, JObjectArray, JByteArray};
        use jni::sys::{jdouble, jfloat, jlong, jint, jboolean, jsize};
        use jni::JNIEnv;

        /// Helper function which tries to get a meaningful description from a panic-error.
        fn any_to_string(any: &Box<dyn std::any::Any + Send>) -> String {
            if let Some(s) = any.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = any.downcast_ref::<String>() {
                s.clone()
            } else if let Some(error) = any.downcast_ref::<Box<dyn std::error::Error + Send>>() {
                format!("{error:#?}")
            } else {
                "Unknown error occurred".to_string()
            }
        }
    )
}

/// Deals exclusively with `proc_macro2::TokenStream` instead of `proc_macro::TokenStream`,
/// allowing it and all interior functionality to be unit tested.
fn generate_inner(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let attr_span = attr.span();
    let item_span = item.span();

    // ensure this is applied to a `mod` item
    let mut module: syn::ItemMod = match syn::parse2(item) {
        Ok(f) => f,
        Err(_e) => {
            return Err(syn::Error::new(
                item_span,
                "The attribute can only be applied to `mod` items",
            ))
        }
    };

    // extract the namespace from the macro attribute
    let namespace = match syn::parse2::<syn::LitStr>(attr) {
        Ok(n) => n,
        Err(_e) => {
            return Err(syn::Error::new(
                attr_span,
                "The attribute must have a single string literal supplied to specify the namespace",
            ))
        }
    }
    .value();
    // TODO: validate that the namespace name is valid
    // if !valid_namespace(&namespace) {
    //     return syn::Error::new(attr_span, "Invalid package namespace supplied to `jni_fn` attribute")
    //         .to_compile_error();
    // }

    // go through each function and modify them to be JNI compatible
    if let Some((_, content)) = &mut module.content {
        for item in content.iter_mut() {
            match item {
                syn::Item::Fn(function) => expand_fn(function, &namespace)?,
                syn::Item::Use(_) => {}
                _ => {
                    return Err(syn::Error::new(
                        item_span,
                        "The attribute can only be applied to `mod` items containing functions",
                    ))
                }
            }
        }
        // insert jni prelude at the top of the module
        let prelude = mod_prelude();
        content.insert(0, syn::Item::Verbatim(prelude));
    }

    Ok(module.into_token_stream())
}

fn expand_fn(function: &mut syn::ItemFn, namespace: &str) -> syn::Result<()> {
    // first, make sure the function has the correct attributes, ABI and rename to be JNI
    // compatible
    add_no_mangle(function)?;
    allow_non_snake_case(function)?;
    ensure_correct_abi(function)?;
    ensure_function_is_public(function)?;
    rename_to_jni(function, namespace)?;
    drop_public_name_attribute(function)?;

    // now we can deal with the arguments and return types! (i.e. insert the JNIEnv argument
    // automatically etc

    let fn_prelude = transform_input_arguments(function)?;
    add_jni_env_arguments(function)?;

    //  transform the return type and capture the postlude needed
    let return_type = transform_return_type(function)?;

    // finally we can insert the prelude and postlude surrounding the function body
    if !function.block.stmts.is_empty() {
        // create an identity where the original function result will go into
        let result_ident = syn::Ident::new("return_value", Span::call_site());

        // add the conversion of the return value to JNI
        let fn_postlude = return_type.fn_postlude(&result_ident);
        let panic_value = return_type.to_rust_panic_value();
        let original_return_type = return_type.to_rust_type();

        let original_block = &function.block;
        function.block = syn::parse_quote! ({
            #fn_prelude
            let result = std::panic::catch_unwind(|| {
                let output: #original_return_type = #original_block;
                output
            });
            let #result_ident = result.unwrap_or_else(|cause| {
                if let Err(e) = env.throw_new("java/lang/RuntimeException", &any_to_string(&cause)) {
                    eprintln!("Could not raise exception: {e:#?}");
                }
                #panic_value
            });
            #fn_postlude
        });
    }

    Ok(())
}

/// replaces the arguments by the JNI types and inserts the JNIEnv and JClass variables
/// returns the function prelude
fn transform_input_arguments(function: &mut syn::ItemFn) -> syn::Result<TokenStream> {
    let mut function_prelude = TokenStream::new();

    for arg in function.sig.inputs.iter_mut() {
        // get our internal representation of the argument type
        let (name, argtype) = ArgumentType::parse(arg)?;

        // create a new identity for the name of the argument (same as existing)
        let name_ident = syn::Ident::new(&name, Span::call_site());

        // get the type we need for interfacing with JNI
        let jnitype = argtype.to_jni_type();

        // finally construct the new argument definition and assign it to the argument
        let new_arg: syn::FnArg = syn::parse_quote! {#name_ident: #jnitype};
        *arg = new_arg;

        // extend the prelude with what we need for converting each of the arguments
        // from their JNI types to the rust types (eg for String)
        let pre = argtype.fn_prelude(&name_ident);
        function_prelude = quote!(
            #function_prelude
            #pre
        );
    }

    Ok(function_prelude)
}

fn add_jni_env_arguments(function: &mut syn::ItemFn) -> syn::Result<()> {
    // add the lifetime to the function signature
    function.sig.generics = syn::parse_quote!(<'local>);

    // add the JNIEnv and JClass arguments
    function
        .sig
        .inputs
        .insert(0, syn::parse_quote!(mut env: JNIEnv<'local>));
    function.sig.inputs.insert(1, syn::parse_quote!(_class: JClass<'local>));

    Ok(())
}

fn transform_return_type(function: &mut syn::ItemFn) -> syn::Result<ReturnType> {
    // make sure the output type is supported
    let rtype = ReturnType::parse(&function.sig.output)?;

    // update return signature to be JNI compatible
    function.sig.output = rtype.to_jni_type();

    Ok(rtype)
}

/// Adds the #[no_mangle] attribute to a function
fn add_no_mangle(function: &mut syn::ItemFn) -> syn::Result<()> {
    function.attrs.push(syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::Path(syn::parse_str("no_mangle").unwrap()),
    });

    Ok(())
}

/// adds an attribute to allow non_snake_case lint
fn allow_non_snake_case(function: &mut syn::ItemFn) -> syn::Result<()> {
    function.attrs.push(syn::Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(syn::MetaList {
            path: syn::parse_str("allow").unwrap(),
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            tokens: quote::quote! { non_snake_case },
        }),
    });

    Ok(())
}

/// ensures that the function has the correct ABI ("system")
fn ensure_correct_abi(function: &mut syn::ItemFn) -> syn::Result<()> {
    if function.sig.abi.is_some() {
        return Err(syn::Error::new(
            function.sig.abi.span(),
            "Don't specify an ABI for the functions functions - the correct ABI will be added automatically",
        ));
    }

    function.sig.abi = Some(syn::Abi {
        extern_token: Default::default(),
        name: Some(syn::LitStr::new("system", function.sig.ident.span())),
    });

    Ok(())
}

/// ensures that the function is public
fn ensure_function_is_public(function: &mut syn::ItemFn) -> syn::Result<()> {
    if !matches!(function.vis, syn::Visibility::Public(_)) {
        return Err(syn::Error::new(
            function.vis.span(),
            "functions must have public visibility (`pub`)",
        ));
    }

    Ok(())
}

/// renames the function to have a name matching the JNI standards
fn rename_to_jni(function: &mut syn::ItemFn, namespace: &str) -> syn::Result<()> {
    let orig_fn_name = function.sig.ident.to_string();

    // suffix with "Jni" in order to avioid name clashes
    let orig_fn_name = format!("{orig_fn_name}Jni");

    function.sig.ident = syn::Ident::new(
        &create_jni_function_name(namespace, &orig_fn_name),
        function.sig.ident.span(),
    );

    Ok(())
}

/// this tries to parse the public name of the function, and at the same time remove it
/// from the attributes (since build script has already run in this stage)
fn drop_public_name_attribute(function: &mut syn::ItemFn) -> syn::Result<()> {
    // first try to parse all the attributes, and propagate any errors with the qestion mark
    // operator
    let _ = function
        .attrs
        .iter()
        .map(jnigen_common::get_public_name)
        .collect::<syn::Result<Vec<Option<String>>>>()?;

    // now we can safely filter the list and unwrap any errors
    function
        .attrs
        .retain(|attr| jnigen_common::get_public_name(attr).unwrap().is_none());

    Ok(())
}

/// Combines the namespace with the function name to create a valid JNI name.
/// Note: This does _not_ convert the name to snameCase.
/// Requirements:
///     - Underscores in the original name and namespace should be replaced by "_1"
///     - Dot separator in namespace need to be replaced by underscore
///     - Should start with "Java_"
///
fn create_jni_function_name(namespace: &str, function_name: &str) -> String {
    let namespace_underscored = namespace.replace('_', "_1").replace('.', "_");
    let fn_name_underscored = function_name.replace('_', "_1");
    format!("Java_{}_{}", namespace_underscored, fn_name_underscored)
}

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::generate_inner;
    use super::mod_prelude;

    #[test]
    fn codegen_empty() {
        let attr = quote! {"com.example.Bar"};
        let source = quote! {
            mod jni {}
        };

        let expanded = generate_inner(attr, source).unwrap();
        let prelude = mod_prelude();

        assert_eq!(
            format!("{}", expanded),
            format!(
                "{}",
                quote::quote! {
                    mod jni {
                        #prelude
                    }
                }
            )
        );
    }

    #[test]
    fn codegen_simple_function() {
        let attr = quote! {"com.example.Bar"};
        let source = quote! {
            mod jni {
                pub fn myFunction() {
                }
            }
        };

        let expanded = generate_inner(attr, source).unwrap();
        let prelude = mod_prelude();
        assert_eq!(
            format!("{}", expanded),
            format!(
                "{}",
                quote::quote! {
                    mod jni {
                        #prelude
                        #[no_mangle]
                        #[allow(non_snake_case)]
                        pub extern "system" fn Java_com_example_Bar_myFunctionJni<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>) {

                        }

                    }
                }
            )
        );
    }
}
