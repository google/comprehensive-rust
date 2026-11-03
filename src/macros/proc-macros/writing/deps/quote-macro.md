---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The `quote!` macro

The **`quote`** crate provides the `quote!` macro, which is a powerful
templating engine for generating Rust code. It lets you write standard Rust
syntax and interpolate existing code from variables in scope:

```rust,ignore
use quote::quote;

let name = ast.ident; // e.g. "Student"

// Generate a TokenStream using standard Rust syntax as a template:
let expanded = quote! {
    impl std::fmt::Display for #name {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "My name is {}", stringify!(#name))
        }
    }
};

// Convert to proc_macro::TokenStream:
expanded.into()
```

- **Interpolation:** Use `#var_name` to insert a variable (like an identifier,
  type, or other token stream) directly into the template.
- **Repetitions:** Use `#(#var_list),*` to expand a list of variables separated
  by commas, similar to how repetitions work in declarative macros.

<details>

- Note that `quote!` returns a `proc_macro2::TokenStream`. We call `.into()` at
  the end of our procedural macro function to convert it to a standard
  `proc_macro::TokenStream` for the compiler.
- Highlight how clean this makes code generation. Instead of manually pushing
  string slices or tokens, you write literal Rust code blocks with templates.
- Mention that repetition is often useful when iterating over all fields of a
  struct.

</details>
