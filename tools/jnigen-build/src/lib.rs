#![allow(clippy::expect_used)]
use quote::ToTokens;
use std::fmt::Write;
use std::path::Path;
use std::process;
use std::str::FromStr;
use std::{fs::File, io::Read};
use syn::Attribute;
use syn::spanned::Spanned;

use jnigen_common::{ArgumentType, ReturnType};

pub fn generate(filename: &str, library_name: &str, java_src_folder: &str) {
    // load the file and parse its contents into a syntax tree
    let mut file = File::open(filename).expect("Unable to open file");
    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");
    let file = syn::parse_file(&src).expect("Unable to parse file");

    let bindings = parse_file(&file);

    match bindings {
        Ok(parsed) => write_output(java_src_folder, &parsed, library_name),
        Err(e) => {
            print_error(&src, &e);
            process::exit(1);
        }
    }
}

fn parse_file(file: &syn::File) -> syn::Result<ParsedBindings> {
    for item in &file.items {
        if let syn::Item::Mod(module) = item {
            // does it have the right attribute?

            if let Some(attr) = module.attrs.iter().find(|a| {
                let attrib = a.path().to_token_stream().to_string();
                attrib == "jnigen_macro :: generate"
            }) {
                // extract the namespace from the macro attribute
                let namespace = attr
                    .parse_args::<syn::LitStr>()
                    .unwrap_or_else(|_| {
                        panic!(
                            "The attribute must have a single string literal supplied to specify the namespace {:?}",
                            attr.span()
                        )
                    })
                    .value();
                return parse_module(module, &namespace);
            }
        }
    }
    Err(syn::Error::new(
        file.span(),
        "No module with the attribute `jnigen_macro::generate` found in the file",
    ))
}

/// Somewhat petty-prints the `syn` error together with the source lines.
/// A bit hacky and no special support for multiline errors but it gets the job done.
fn print_error(source: &str, error: &syn::Error) {
    let span = error.span();
    let start = span.start();
    let end = span.end();

    let lines: Vec<&str> = source.split('\n').collect();

    let start_line = start.line.saturating_sub(2);
    let end_line = usize::min(lines.len(), end.line + 2);

    for (l, line) in lines.iter().enumerate().take(end_line).skip(start_line) {
        if l == start.line {
            eprintln!(
                "{}{} - {}",
                " ".repeat(start.column),
                "^".repeat(end.column - start.column),
                error
            )
        }
        eprintln!("{}", line);
    }
}

fn parse_module(module: &syn::ItemMod, namespace: &str) -> syn::Result<ParsedBindings> {
    let mut functions = Vec::new();
    let doc_lines = extract_doc_attribute_lines(&module.attrs);

    // go through each function and modify them to be JNI compatible
    if let Some((_, content)) = &module.content {
        for item in content.iter() {
            match item {
                syn::Item::Fn(function) => functions.push(Function::parse(function)?),
                syn::Item::Use(_) => {}
                unsupported => {
                    return Err(syn::Error::new(
                        unsupported.span(),
                        "The attribute can only be applied to `mod` items containing functions",
                    ));
                }
            }
        }
    }

    Ok(ParsedBindings {
        doc_lines,
        namespace: namespace.parse().expect("class name should be valid"),
        functions,
    })
}

#[derive(Debug)]
struct ParsedBindings {
    doc_lines: Vec<String>,
    namespace: PackageClassName,
    functions: Vec<Function>,
}

#[derive(Debug)]
struct Function {
    doc_lines: Vec<String>,
    name: String,
    public_name: Option<String>,
    arguments: Vec<(String, ArgumentType)>,
    return_type: ReturnType,
    is_deprecated: bool,
}

impl Function {
    fn parse(function: &syn::ItemFn) -> syn::Result<Self> {
        let name = function.sig.ident.to_string();

        // parse the public name attribute (if it exists)
        let mut public_name = None;
        let mut is_deprecated = false;
        for attr in &function.attrs {
            if let Some(name) = jnigen_common::get_public_name(attr)? {
                public_name = Some(name);
            }
            if attr.path().is_ident("deprecated") {
                is_deprecated = true;
            }
        }

        let doc_lines = extract_doc_attribute_lines(&function.attrs);

        let arguments = function
            .sig
            .inputs
            .iter()
            .map(ArgumentType::parse)
            .collect::<syn::Result<Vec<(String, ArgumentType)>>>()?;

        let return_type = ReturnType::parse(&function.sig.output)?;

        Ok(Self {
            doc_lines,
            name,
            public_name,
            arguments,
            return_type,
            is_deprecated,
        })
    }
}

/// Extract the docstring by parsing all #[doc = ""] attributes (which is what the `///`
/// comments are represented as)
fn extract_doc_attribute_lines(attributes: &[Attribute]) -> Vec<String> {
    let mut doc_lines = Vec::new();
    for attr in attributes {
        if attr.path().is_ident("doc") {
            if let syn::Meta::NameValue(meta) = &attr.meta {
                if let syn::Expr::Lit(expr) = &meta.value {
                    if let syn::Lit::Str(lit_str) = &expr.lit {
                        doc_lines.push(lit_str.value().trim().to_string());
                    }
                }
            }
        }
    }
    doc_lines
}

// the documentation strings as java comments
fn write_doc_lines_as_javadoc_comment<B: Write>(mut buffer: B, lines: &[String], indentation: &str) {
    if !lines.is_empty() {
        writeln!(&mut buffer, "{indentation}/**").unwrap();
        for line in lines {
            writeln!(&mut buffer, "{indentation} * {line}").unwrap();
        }
        writeln!(&mut buffer, "{indentation} */").unwrap();
    }
}

impl ParsedBindings {
    fn generate_java_class(&self, library_name: &str) -> String {
        // split namespace into class name and path
        let path = self.namespace.package_path();
        let class_name = self.namespace.class_name();
        let mut buffer = String::new();

        // intro
        writeln!(&mut buffer, "package {path};\n").unwrap();

        write_doc_lines_as_javadoc_comment(&mut buffer, &self.doc_lines, "");

        writeln!(
            &mut buffer,
            "public class {class_name} {{
    static {{
        System.loadLibrary(\"{library_name}\");
    }}
"
        )
        .unwrap();

        for fun in &self.functions {
            // prepare some names / type lists
            let public_name = match &fun.public_name {
                Some(name) => name,
                None => &fun.name,
            };

            let jni_name = format!("{}Jni", fun.name);
            let return_type = fun.return_type.to_java_type();
            let argument_list = fun
                .arguments
                .iter()
                .map(|(name, arg)| format!("{} {name}", arg.to_java_type()))
                .collect::<Vec<String>>()
                .join(", ");
            let parameter_list = fun
                .arguments
                .iter()
                .map(|(name, _)| name.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            let deprecated = if fun.is_deprecated { "@Deprecated " } else { "" };

            // the JNI decrataion
            writeln!(
                &mut buffer,
                "    private static native {return_type} {jni_name}({argument_list});\n"
            )
            .unwrap();

            write_doc_lines_as_javadoc_comment(&mut buffer, &fun.doc_lines, "    ");

            // the public function
            if !fun.return_type.has_java_return_value() {
                writeln!(
                    &mut buffer,
                    "    {deprecated}public {return_type} {public_name}({argument_list}) throws Exception {{
        {jni_name}({parameter_list});
    }}\n"
                )
                .unwrap();
            } else {
                writeln!(
                    &mut buffer,
                    "    {deprecated}public {return_type} {public_name}({argument_list}) throws Exception {{
        return {jni_name}({parameter_list});
    }}\n"
                )
                .unwrap();
            }
        }

        // outro
        writeln!(&mut buffer, "\n}}").unwrap();

        buffer
    }
}

fn write_output(java_src_folder: &str, parsed: &ParsedBindings, library_name: &str) {
    let java_source = parsed.generate_java_class(library_name);

    let src_folder = Path::new(java_src_folder);
    let mut buff = src_folder.to_path_buf();
    buff.extend(parsed.namespace.package_parts());
    let folder = buff.as_path();

    // make sure folder exitsts, or create it
    std::fs::create_dir_all(folder).expect("Could not create output directory");

    // create the file and output the source
    let class_file = folder.join(parsed.namespace.class_name()).with_extension("java");

    std::fs::write(&class_file, java_source).expect("Could not write java source");

    // make sure the build script is rerun if the output file changes (eg. if someone edits it)
    println!("cargo:rerun-if-changed={}", class_file.display());
}

#[derive(Debug)]
struct PackageClassName {
    /// The dot-separated parts of the namespace
    parts: Vec<String>,
}

impl PackageClassName {
    pub fn package_path(&self) -> String {
        self.package_parts().join(".")
    }
    pub fn package_parts(&self) -> &[String] {
        &self.parts[0..self.parts.len() - 1]
    }
    pub fn class_name(&self) -> &str {
        &self.parts[self.parts.len() - 1]
    }
}

impl FromStr for PackageClassName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('.').collect();

        if parts.is_empty() {
            return Err(String::from("Empty string is not a valid namespace!"));
        }

        let parts = parts.iter().map(|s| s.to_string()).collect();

        Ok(Self { parts })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_split_namespace() {
        let ps = PackageClassName::from_str("com.example.Class").unwrap();
        assert_eq!(&ps.package_path(), "com.example");
        assert_eq!(ps.class_name(), "Class");

        let ps = PackageClassName::from_str("com.Class").unwrap();
        assert_eq!(&ps.package_path(), "com");
        assert_eq!(ps.class_name(), "Class");

        let ps = PackageClassName::from_str("Class").unwrap();
        assert_eq!(&ps.package_path(), "");
        assert_eq!(ps.class_name(), "Class");
    }

    #[test]
    fn test_docstrings() {
        let input: syn::File = syn::parse_quote! {
            /// Example multi-line
            /// doc comment that turns into
            /// a javadoc comment.
            #[jnigen_macro::generate("com.jnigen.tests.MyTestClass")]
            mod ffi {
                /// Function comment
                pub fn function() {}
            }
        };

        let bindings = parse_file(&input).unwrap();

        let java_code = bindings.generate_java_class("bindings");
        assert_eq!(
            java_code,
            r#"package com.jnigen.tests;

/**
 * Example multi-line
 * doc comment that turns into
 * a javadoc comment.
 */
public class MyTestClass {
    static {
        System.loadLibrary("bindings");
    }

    private static native void functionJni();

    /**
     * Function comment
     */
    public void function() throws Exception {
        functionJni();
    }


}
"#
        );
    }
}
