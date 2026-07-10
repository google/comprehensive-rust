---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The `syn` and `quote` Crates

Working directly with `TokenStream`s is low-level and tedious. To write
procedural macros effectively, authors often rely on **`syn`** (a parser for
Rust syntax) and **`quote`** (which constructs `syn` values from literal code
fragments) to do the heavy lifting of parsing and code generation.

### The workflow of a typical procedural macro is:

1. **Input:** Accept a raw `proc_macro::TokenStream` from the compiler.
2. **Parsing (`syn`):** Convert the token stream into a structured, type-safe
   Abstract Syntax Tree (AST) representing Rust language items (e.g., structs,
   fields, functions).
3. **Processing:** Inspect the AST struct and perform your custom logic to
   decide what code needs to be generated.
4. **Generation (`quote`):** Use a templated macro to generate a new
   `proc_macro2::TokenStream`.
5. **Output:** Convert it back to `proc_macro::TokenStream` and return it to the
   compiler.

We will look at how `syn` parses code and how `quote` generates code.
