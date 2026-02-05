# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Consuming Builder:** The methods on `PackageBuilder` take `mut self` and
  return `Self`. This pattern transfers ownership of the builder into the
  method, modifies it, and returns ownership back to the caller. This allows
  method chaining (e.g., `.version(...).authors(...)`).
- **`impl Into<String>`:** Arguments like `name` and `version` use
  `impl Into<String>`. This allows the caller to pass anything that can be
  converted into a `String`, such as a string literal (`&str`) or an owned
  `String`. It makes the API flexible.
- **Cloning:** In `as_dependency`, we must `clone()` the name and version. The
  method takes `&self` (a shared reference), so we cannot move fields out of the
  package. Since `Dependency` owns its strings, we must create new copies.
- **Tuple Struct Wrapper:** `PackageBuilder` is a tuple struct wrapping
  `Package`. We access the inner package via `self.0`. This hides the
  implementation details of `Package` while it's being built.

<details>

- Discuss why `as_dependency` requires cloning. If it took `self` (by value), it
  could move the fields, but that would consume the `Package`, preventing
  further use.
- The `dependencies` vector owns the `Dependency` structs.

</details>
