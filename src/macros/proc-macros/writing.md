---
minutes: 6
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Writing Procedural Macros

Writing procedural macros is basically about learning how to work with the
standard helper libraries designed specifically to parse and generate Rust code.

Because dealing with raw, low-level `TokenStream`s manually is tedious and
difficult, we use a standard suite of specialized helper libraries:

1. **`proc_macro`:** Provided by the compiler to interface token streams.
2. **`proc_macro2`:** Wraps `proc_macro` to allow unit testing and
   out-of-compiler executions.
3. **`syn`:** A complete parser library that parses Rust token streams into
   structured Abstract Syntax Tree (AST) structures.
4. **`quote`:** A templated code generation library that turns your Rust
   structures back into token streams.

We will look at how each of these crates builds on top of one another.

<details>

- Consider how writing a proc macro is essentially writing a mini-compiler: you
  parse a string of tokens (lexical analysis), match its structure (parsing),
  and generate a new string of tokens (transformation and code emission).
- Reassure students that you rarely ever work with raw tokens directly. The
  `syn` and `quote` crates do 95% of the heavy parsing and formatting work for
  you, allowing you to write more-readable macro code.

</details>
