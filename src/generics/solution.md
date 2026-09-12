<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

The solution uses trait bounds to constrain the generic type `T`:

- **Trait Bounds:** The `<T: Ord>` syntax requires that any type used with `min`
  must implement the `Ord` trait, which guarantees a total ordering.
- **Comparison:** The `cmp` method returns an `Ordering` enum (`Less`, `Equal`,
  or `Greater`), which we handle with pattern matching.

<details>

- **`Ord` vs. `PartialOrd`:** Rust distinguishes between types that have a total
  order (`Ord`) and those that only have a partial order (`PartialOrd`).
  Floating-point numbers (`f32`, `f64`) only implement `PartialOrd` because
  `NaN` cannot be compared. Consequently, this `min` function cannot be called
  with floats.
- **Standard Library:** In real-world code, one would typically use the built-in
  `std::cmp::min` function or the `.min()` method available on many types.

</details>
