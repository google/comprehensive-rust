---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Attribute Macros

**Attribute procedural macros** are decorated with the `#[proc_macro_attribute]`
attribute. They allow you to define custom attributes that completely replace
the items they are attached to:

```rust,ignore
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Process parameters in `attr` (e.g., GET, "/path")
    // Process the attached function/item in `item`
    // Return the new, fully rewritten item...
    item
}
```

- **Two Parameters:**
  1. `attr`: The token stream containing any metadata parameters passed into the
     attribute (e.g., `GET, "/"` inside `#[route(GET, "/")]`).
  2. `item`: The token stream representing the entire attached item (e.g., a
     function or struct definition).
- **Replacement:** The returned token stream entirely replaces the input `item`
  in the AST.

<details>

- Explain that because attribute macros completely replace the input item, you
  must make sure to output the rewritten item (along with any generated
  auxiliary code) so that it remains visible to the rest of the program, unless
  your specific intention is to delete it!
- Discuss how popular libraries (such as Axum or Rocket) use attribute macros to
  register route functions into global dispatch tables, or rewrite signatures
  for dependency injection.

</details>
