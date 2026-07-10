---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Macros In Rust

Rust macros provide structured code generation. There are **three kinds** of
macros, and **two ways** they are implemented.

### Kinds of Macros

- **Derive Macros:** Added to type definitions (structs, enums, unions) to
  auto-implement traits (e.g., `#[derive(Default)]`).
- **Function-Like Macros:** Invoked with an exclamation mark in
  item/statement/expr context (e.g., `println!("Hello!")`, `vec![1, 2, 3]`, or
  `include_bytes!("manifest.bin")`).
- **Attribute Macros:** Attached as custom attributes to any item, like
  functions or modules (e.g., `#[tokio::main]`).

### Implementation Forms

- **Declarative Macros (AKA "Macros By Example"):** Part of the language itself,
  based on pattern matching; these can only be used to define function-like
  macros.
- **Procedural Macros:** Rust functions running as compiler plugins that
  transform token streams. These can perform arbitrary operations at compile
  time, including I/O if desired.

<details>

- Highlight that students have already used function-like macros (like
  `println!`, `vec!`, `format!`) and derive macros (like
  `#[derive(Clone, Debug)]`).
- Explain that attribute macros are very popular in libraries like Tokio or
  Axum, often transforming functions similar to Python decorators.

</details>
