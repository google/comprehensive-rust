---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Procedural Macros

**Procedural macros** are far more powerful than declarative macros. Instead of
matching syntax templates, they are literal compiler plugins:

- They are implemented as functions that receive one or two **token streams**
  and return a **token stream**.
- They are written in standard, fully featured Rust.
- They have complete programmatic access to the incoming token stream, allowing
  you to parse, modify, or generate complex code structures.
- They require a dedicated library crate configured with `proc-macro = true` in
  its `Cargo.toml`.

We will explore how procedural macros are defined, how they compile, and how to
write them.

<details>

- Explain that procedural macros are called "procedural" because they execute
  standard Rust procedures (functions) to transform code at compile time.
- Emphasize that because they are compiled as libraries and loaded by the
  compiler during build time, they have a heavier impact on compilation speed
  compared to declarative macros.
- This slide serves as a transition into the detailed mechanics of procedural
  macros.

</details>
