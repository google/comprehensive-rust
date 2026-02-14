# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

The solution demonstrates ownership transfer and flexible API design:

- **Consuming Builder:** Methods take `mut self` and return `Self`. This pattern
  transfers ownership into the method, modifies the state, and returns it,
  enabling fluid method chaining.
- **`impl Into<String>`:** This trait bound allows callers to pass either `&str`
  or `String`, with `into()` performing the conversion to an owned string only
  when necessary.
- **Cloning in `as_dependency`:** Since `as_dependency` takes `&self`, it cannot
  move fields out of the `Package`. We must `clone()` the strings to create an
  owned `Dependency`.

<details>

- **Why `mut self`?** A consuming builder is idiomatic for one-off construction.
  If we used `&mut self`, the methods would return `&mut Self`, and the final
  `.build()` call would still need to consume the builder or clone the entire
  state.
- **Inner Representation:** `PackageBuilder` uses a tuple struct to wrap
  `Package`, providing a clean interface that hides the inner data structure
  during construction.

</details>
