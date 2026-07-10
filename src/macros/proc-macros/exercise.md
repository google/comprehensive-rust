---
minutes: 20
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: `Display` Derive

In this exercise, you will walk through the conceptual implementation of a
custom **`Display` derive macro** using the `syn` and `quote` crates.

Since we are running this in a single-file environment without a separate
`proc-macro = true` crate setup, we will write a normal Rust function that takes
simulated token streams (using `proc_macro2` and `quote!`), parses them with
`syn`, generates the output with `quote!`, and asserts that the generated code
is correct.

### Your task:

Implement the `derive_display_impl` function to:

1. Parse the incoming token stream `input` into a `syn::DeriveInput` struct.
2. Extract the name (identifier) of the struct.
3. Generate and return a token stream implementing `std::fmt::Display` for that
   struct name, which formats the struct using its name (e.g., printing
   `"MyAwesomeType"`).

This will rely on a `Cargo.toml` resembling:

```toml
{{#include exercise/Cargo.toml}}
```

```rust,compile_fail,editable
{{#include exercise/exercise.rs:Derive}}
    // TODO: Parse the input TokenStream into DeriveInput.
    // TODO: Extract the identifier of the struct.
    // TODO: Generate the impl std::fmt::Display using quote!.
    todo!()
{{#include exercise/exercise.rs:rest}}
```

<details>

- `parse2(input)` is the `proc_macro2` version of `parse_macro_input!`, which we
  can use for unit testing or within normal Rust functions.
- The return value of `derive_display_impl` is a `proc_macro2::TokenStream`,
  which can be converted to a string using `to_string()` for easy debugging.

</details>
