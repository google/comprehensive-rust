---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# What Are Macros For

Macros are a powerful tool, but carry their own complexity and should be used
only when worthwhile. They are used to solve problems that cannot be easily
solved with standard Rust language features:

- **Reducing Boilerplate:** Implementing traits via `#[derive(...)]` or reducing
  syntactic repetition.
- **Variadic Interfaces:** Functions that take a variable number of arguments of
  different types (like `println!` and `vec!`).
- **Domain-Specific Languages (DSLs):** Creating custom syntax inside Rust
  (e.g., `println!`'s format strings, or macros that statically expand HTML
  templates or parse SQL queries).
- **Compile-Time Computation:** Inspecting configuration files, environment
  variables, or data at build time (e.g., `include_str!`, `env!`).

Because each macro can define its own input syntax and can perform operations
not available to normal Rust code, macros can make code harder to read than it
would be otherwise. Try to avoid using macros when the same abstraction can be
achieved in another way, and when defining macros, try to align with syntactic
and naming patterns followed in non-macro Rust code.

<details>

- Contrast with limitations of normal Rust code: functions have a fixed
  signature (fixed number of arguments of specific types) and cannot generate
  new types or modules.

</details>
