---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Procedural Macros In The Wild

Many of the most popular and foundational libraries in the Rust ecosystem rely
heavily on procedural macros to provide elegant, declarative, and type-safe
APIs:

- **Serde (`#[derive(Serialize, Deserialize)]`):** Automatically generates code
  to convert Rust types to/from JSON, YAML, or binary formats based on field
  names and structural nesting.
- **Tokio (`#[tokio::main]`):** Sets up runtime environments, wrapping standard
  main entry points asynchronously.
- **Clap (`#[derive(Parser)]`):** Automatically builds complete, robust CLI
  command-line argument parsers based on struct declarations and doc comments.
- **SQLx (`query!`):** Validates SQL query strings against a live database at
  build time, converting database schemas directly to static Rust types.

<details>

- Highlight the impact of procedural macros for Rust's
  ergonomics--serialization, for example, requires explicit compiler support in
  many languages.
- SQLx's compile-time query verification is a prime example of compile-time code
  validation that is only possible with procedural macros.

</details>
