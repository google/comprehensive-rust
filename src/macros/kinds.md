---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Where Macros Can Apply

Macros can be attached to or invoked at different positions in your code. There
are three major cases to consider:

- **Function-Like Macros:** Invoked as expressions, statements, types, patterns,
  or items, expanding to any syntax that could occupy the invocation position.
- **Derive Macros:** Attached specifically to `struct`, `enum`, or `union`
  items, to generate trait impls based on the type definition.
- **Attribute Macros:** Attached to any standard Rust `item` (including
  functions, modules, structs, etc.), allowing complete replacement of the
  attached item.

We will now see how each of these looks and behaves.

<details>

- Mention that "items" in Rust are top-level declarations (which can nest!) such
  as structs, modules, traits, and functions.
- Clarify that some macro positions are more restricted than others: e.g.,
  derive macros cannot modify the structs they are attached to, but only append
  new implementations alongside; meanwhile, attribute macros can rewrite their
  targets entirely.

</details>
