---
minutes: 5
---

# Composition over Inheritance

```rust
pub struct Uuid([u8; 16]);

pub struct Address {
    street: String,
    city_or_province: String,
    code: String,
    country: String,
}

pub struct User {
    id: Uuid,
    address: Address,
}
```

<details>
- Rather than mixins or inheritance, we compose types by creating fields of different types.

This has downsides, largely in ergonomics of field access, but gives developers
a lot of control and clarity over what a type does and it has access to.

- When deriving traits, make sure all the field types of a struct or variant
  types of an enum implement that trait. Derive macros often assume all types
  that compose a new type implement that trait already.

</details>
