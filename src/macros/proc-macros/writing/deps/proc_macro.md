---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# The `proc_macro` Crate

The compiler-provided `proc_macro` crate defines the primary low-level types for
procedural macro manipulation. These are the actual types underlying the tokens
we match in declarative macros:

- **`TokenStream`:** An iterator over a sequence of `TokenTree` values.
- **`TokenTree`:** An enum representing individual lexical units. It has four
  variants:
  - `Group`: A bracketed group of tokens (e.g., `(a + b)`, `{ ... }`).
  - `Ident`: An identifier or keyword (e.g., `foo`, `fn`, `let`, `false`).
  - `Punct`: A single punctuation character (e.g., `+`, `,`, `:`, but _not_
    combined operators like `+=`, nor brackets).
  - `Literal`: A literal value (e.g., `"hello"`, `42`, `3.14`).

```rust,ignore
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}
```

<details>

- Discuss that `proc_macro` is a built-in crate, so you don't need to add it to
  your `Cargo.toml` dependencies (just defining `proc-macro = true` makes it
  automatically available).
- `TokenStream` can be converted to and from strings via `to_string()` and
  `parse()`, but doing so breaks its relation to its original source code
  location (`Span`), which should be preserved so compiler error messages can
  point at the relevant code.

</details>
