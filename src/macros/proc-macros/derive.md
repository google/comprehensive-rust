---
minutes: 8
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Derive Macros

**Derive procedural macros** are decorated with the
`#[proc_macro_derive(TraitName)]` attribute. They take in the token stream of a
type definition and return a token stream containing additional implementations:

```rust,ignore
use proc_macro::TokenStream;

#[proc_macro_derive(MyDebug)]
pub fn derive_my_debug(item: TokenStream) -> TokenStream {
    // Parse the struct/enum item, generate `MyDebug` implementation...
    TokenStream::new()
}
```

- **Additive Output:** The returned token stream is compiled alongside the type
  definition in the same scope. It does not replace the type definition itself.
- **Helper Attributes:** Derive macros can declare optional helper attributes
  that can be used on struct fields or enum variants, configured in the derive
  declaration: `#[proc_macro_derive(MyTrait, attributes(my_helper))]`

<details>

- The `item` TokenStream represents the entire struct, enum, or union
  definition, including its attributes, generic parameters, braces, and fields.
- Discuss helper attributes: helper attributes are matched by the derive macro
  to let users customize the code generation (e.g.,
  `#[my_helper(rename = "field")]`).
- Note that the argument to `proc_macro_derive` dictates how this derive macro
  is imported and triggered, but does not actually constrain which items it
  generates `impl` blocks for. Some derive macros may impl multiple traits (as
  does `thiserror::Error`, for example); derive macros must be well behaved and
  document their effects.

</details>
