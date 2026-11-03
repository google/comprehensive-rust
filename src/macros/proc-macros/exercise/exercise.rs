// Copyright 2026 Google LLC
// SPDX-License-Identifier: Apache-2.0

// ANCHOR: solution
// ANCHOR: Derive
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{DeriveInput, parse2};

fn struct_fields(ast: &syn::DeriveInput) -> Vec<proc_macro2::TokenStream> {
    if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &ast.data {
        match fields {
            syn::Fields::Named(n) => n
                .named
                .iter()
                .map(|f| {
                    f.ident.as_ref().expect("named field name").to_token_stream()
                })
                .collect(),
            syn::Fields::Unnamed(un) => (0..un.unnamed.len())
                .map(|i| syn::Index::from(i).to_token_stream())
                .collect(),
            syn::Fields::Unit => vec![],
        }
    } else {
        vec![]
    }
}

fn derive_display_impl(input: TokenStream) -> TokenStream {
    // ANCHOR_END: Derive
    // Parse the input TokenStream into DeriveInput
    let ast: DeriveInput = parse2(input).unwrap();

    let fields = struct_fields(&ast);

    // Extract the identifier (name) of the struct
    let name = ast.ident;
    let name_str = name.to_string();

    // Generate the Display implementation using quote!
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{", #name_str)?;
                #(write!(f, "{}: {}", stringify!(#fields), self.#fields)?; )*
                write!(f, "}}")
            }
        }
    }
    // ANCHOR: rest
}

fn main() {
    let input = quote! {
        struct MyAwesomeType {
            field: i32,
        }
    };

    let output = derive_display_impl(input.clone());
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);

    // Check for patterns that should definitely be present in generated code.
    assert!(output_str.contains("impl std :: fmt :: Display for MyAwesomeType"));
    assert!(output_str.contains("\"MyAwesomeType\""));
    assert!(output_str.contains("self . field"));

    // Compile program using the impl to ensure generated code impls `Display`.
    let use_display = quote! {
        fn main() {
            let a = MyAwesomeType { field: 42 };
            println!("{a}");
        }
    };
    let source_str = input.to_string() + &output_str + &use_display.to_string();
    invoke_rustc(&source_str)
        .expect("rustc on generated code produced errors or warnings")
}

fn invoke_rustc(input: &str) -> Result<(), String> {
    use std::io::Write;
    use std::process::Command;

    // Invoke the rustc binary to compile generated code
    let mut child = Command::new("rustc")
        .args(["--verbose", "-"])
        .stdin(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");
    {
        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        stdin.write_all(input.as_bytes()).unwrap();
    }

    // Print the stderr retrieved from rustc
    let stderr = child.wait_with_output().unwrap().stderr;
    let stderr = String::from_utf8_lossy(&stderr).into_owned();
    // Return error on non-empty stderr
    if stderr.is_empty() { Ok(()) } else { Err(stderr) }
}
