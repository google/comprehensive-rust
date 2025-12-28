---
minutes: 5
---

# Conditional Method Implementations

```rust
// No trait bounds on the type definition.
pub struct Value<T>(T);

// Instead bounds are put on the implementations for the type.
impl<T: std::fmt::Display> Value<T> {
    fn log(&self) {
        println!("{}", self.0);
    }
}

// alternatively
impl<T> Value<T> {
    // Specifies the trait bound in a where expression
    fn log_error(&self)
    where
        T: std::error::Error,
    {
        eprintln!("{}", self.0);
    }
}
```

<details>
- When authoring a type with generic parameters, we can write implementations for that type that depend on what the parameters are or what traits they implement.

- These methods are only available when the type meets those conditions.

- For things like ordered sets, where you'd want the inner type to always be
  `Ord`, this is the preferred way of putting a trait bound on a parameter of a
  type.

  We don't put the definition on the type itself as this would cause downstream
  issues for everywhere the type is mentioned with a generic parameter.

  We can maintain invariants just fine with conditional method implementations.

</details>
