---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Attribute Macros

**Attribute macros** are attached as custom attributes to any standard Rust
`item` (including functions, modules, structs, etc.). They have full power to
inspect, rewrite, or completely replace the item they are attached to.

```rust,ignore
// A popular example from the Tokio library:
#[tokio::main]
async fn main() {
    println!("Hello from an async main!");
}
```

- **Universal:** Can be applied to any declaration/item.
- **Transformational:** Unlike derive macros, they can entirely replace their
  target with different code.
- **Parameters:** Can optionally accept custom metadata parameters, e.g.,
  `#[route(GET, "/")]`.
- **Implementation:** Always implemented as procedural macros.

<details>

- Discuss how `#[tokio::main]` converts an `async fn main()` into a synchronous
  `fn main()` that instantiates and runs the Tokio runtime, blocks on the
  future, and executes the async block. This completely rewrites the function
  signature!
- Emphasize that because attribute macros can entirely rewrite items, they are
  extremely powerful but require careful error handling to avoid completely
  obscuring compile errors for the user.

</details>
