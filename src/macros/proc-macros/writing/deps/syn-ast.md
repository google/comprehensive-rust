---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The `syn` AST

The **`syn`** crate provides a complete, robust parser that parses a raw
`TokenStream` into structured, AST types that can represent any element of Rust
code. `syn` has a wide API surface, but it should feel relatively
straightforward as it mirrors the structure of Rust syntax itself. Here we
illustrate the use of `syn` for a Derive macro:

```rust,ignore
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: TokenStream) -> TokenStream {
    // Parse raw tokens into a structured `DeriveInput` AST:
    let ast = parse_macro_input!(input as DeriveInput);

    // Inspect struct metadata:
    let struct_name = ast.ident;
    let generics = ast.generics;

    // ...
}
```

- **`DeriveInput`:** The standard AST structure for a struct, enum, or union.
- **`parse_macro_input!`:** A specialized macro helper provided by `syn` to
  easily parse the input token stream. It automatically handles compile-time
  syntax errors if the input is malformed. This macro accepts the name of the
  type into which to parse the token stream after an `as` separator, notation
  that resembles a Rust cast.

<details>

Explore `DeriveInput`, which three main components:

1. `ident`: The name of the struct/enum.
2. `generics`: Generic parameters, lifetemes, and constraints.
3. `data`: An enum containing `syn::DataStruct`, `syn::DataEnum`, or
   `syn::DataUnion`, which lets you iterate over variants and fields.

</details>
