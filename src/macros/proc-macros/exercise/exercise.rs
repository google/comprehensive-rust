// Copyright 2026 Google LLC
// SPDX-License-Identifier: Apache-2.0

// ANCHOR: solution
// ANCHOR: Derive
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse2};

fn derive_display_impl(input: TokenStream) -> TokenStream {
    // ANCHOR-END: Derive
    // Parse the input TokenStream into DeriveInput
    let ast: DeriveInput = parse2(input).unwrap();

    // Extract the identifier (name) of the struct
    let name = ast.ident;
    let name_str = name.to_string();

    // Generate the Display implementation using quote!
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", #name_str)
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

    let output = derive_display_impl(input);
    let output_str = output.to_string();

    println!("Generated code:\n{}", output_str);

    assert!(output_str.contains("impl std :: fmt :: Display for MyAwesomeType"));
    assert!(output_str.contains("write ! (f , \"{}\" , \"MyAwesomeType\")"));
    println!("Assertion passed successfully!");
}
