---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Fragment Specifiers

**Fragment specifiers** tell the macro compiler what category of syntax to
expect and capture as a meta-variable. Here are the most commonly used
specifiers:

- `ident`: An identifier (i.e., variable or item names such as `foo` or `_x`).
- `expr`: A single expression (e.g., `2 + 2`, `a.method()`, `match x { ... }`).
- `literal`: A literal value (e.g., `"hello"`, `42`, `true`, or `-4.0_f32`).
- `stmt`: A single statement without a semicolon (e.g., `let x = 5`).
- `pat`: A pattern (e.g., `[_, 0, ..]`).
- `ty`: A type (e.g., `i32`, `Vec<String>`, `&str`).
- `path`: A path of segments separated by `::` (e.g.,
  `std::collections::HashMap`).
- `item`: A complete item definition (e.g., `fn foo() {}`, `struct Bar;`).
- `tt`: A single token tree (recall that this is either a single token or a
  bracketed token group).

Less common specifiers include:

- `lifetime`: A lifetime like `'a`.
- `block`: A block of statements enclosed in braces (e.g., `{ let x = 5; x }`).
- `vis`: An optional visibility specifier in an item, e.g. `pub` or
  `pub(crate)`, or nothing.
- `meta`: The possible _contents_ of a `#[...]` attribute (not the full
  attribute with hash and brackets).

There is overlap between a number of these specifiers, e.g. we could write our
own pattern for blocks using `stmt` instead of using `block` directly. In
general, try to reach for the most semantically appropriate fragment specifier
where possible, which will catch more syntactic edge cases and integrate more
seamlessly into the language.

<details>

- Explain that fragment specifiers act like type annotations for macro
  parameters. They restrict what tokens can match.
- For example, if you use `$x:expr`, the input must be parsable as a valid Rust
  expression.
- When the macro is expanded, the compiler inserts the matched AST fragment
  directly, which guarantees semantic structure and operator precedence.
- There are some additional, obscure fragment specifiers such as `expr_2021` and
  `pat_param`.

</details>
