# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **Generic Function:** `min` is a generic function that takes two arguments of
  type `T`.
- **Trait Bounds:** The syntax `<T: Ord>` specifies that `T` must implement the
  `Ord` trait. This is necessary because not all types can be compared (e.g.,
  floating-point numbers in Rust only implement `PartialOrd` due to `NaN`).
- **`Ord` Trait:** The `Ord` trait provides the `cmp` method, which compares two
  values and returns an `Ordering`.
- **`Ordering` Enum:** The result of `cmp` is an enum with variants `Less`,
  `Equal`, and `Greater`. We use `match` to handle these cases.

<details>

- Mention that for floating point numbers, `f64` does not implement `Ord`, so
  this function would not work for them. This is a deliberate design choice in
  Rust to handle `NaN` correctly (NaN != NaN). To handle floats, one would
  typically use `PartialOrd` or a wrapper type.
- Alternatively, `l <= r` works if we use `T: PartialOrd`. However, `Ord` is
  stricter and guarantees a total order, which `cmp` relies on.

</details>
