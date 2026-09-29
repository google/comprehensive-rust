---
minutes: 4
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Function-Like Procedural Macros

**Function-like procedural macros** look and feel like declarative macros to the
caller, but their implementation is a Rust function decorated with the
`#[proc_macro]` attribute:

```rust,ignore
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    // Process input tokens and return output tokens...
    "42".parse().unwrap()
}
```

### Key properties:

- **Declaration:** Decorated with `#[proc_macro]`.
- **Function Signature:** Must accept a single `TokenStream` (the tokens inside
  the brackets of the callsite) and return a `TokenStream`.
- **Flexible Invocation:** Like other function-like macros, callers can invoke
  them with any kind of brackets.

<details>

- Note that the `input` TokenStream contains only the tokens _inside_ the
  brackets, not the macro name or the exclamation mark.
- Emphasize that function-like procedural macros are useful when you need to
  parse complex custom DSLs that cannot be expressed easily with declarative
  `macro_rules!` pattern-matching.
- Mention that they may expand to expressions, statements, or items.

</details>
