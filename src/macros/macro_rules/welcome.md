---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Macros By Example

**Macros by Example** (also known as **Declarative Macros** or `macro_rules!`
macros) are the most common form of macros in Rust and the easiest to write.

- They are defined using the `macro_rules!` construct.
- They allow you to write custom pattern matches on tokens, expanding them into
  standard Rust code.
- They are partially hygienic: local variables and labels defined inside them do
  not leak outside.
- They introduce minimal compile-time overhead and have no external crate
  dependencies.

We will learn how to write, configure, and match patterns in declarative macros.

<details>

- Mention that declarative macros are sometimes called "macros by example"
  because you write an example of the syntax you want to match, and an example
  of the output you want to produce. This is much like C preprocessor macros,
  but with richer pattern matching.
- Highlight that they are defined directly in your standard code, making them
  convenient to add without extra setup.
- This slide acts as a transition into how declarative macros are defined and
  used.

</details>
