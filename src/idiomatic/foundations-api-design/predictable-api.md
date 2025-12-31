---
minutes: 2
---

# Predictable API

Keep your APIs predictable through naming conventions and implementing common
traits.

```rust,compile_fail
/* What traits should this implement? */
pub struct ApiToken(String);

impl ApiToken {
    // What should this method be called?
    pub unsafe fn ____(String) -> ApiToken;
}
```

<details>

- A predictable API is one where a user's can make assumptions about a part of
  the API based on surface-level details like names, types, and signatures.

- We'll be looking at common naming conventions in Rust, which allow users to
  search for methods that fit their needs quickly and be able to understand
  existing code quickly.

- We will also be looking at common traits that types implement, and when to
  implement them for types you define.

</details>
