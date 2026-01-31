# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

This solution highlights the difference between shared and mutable references:

- **Shared References (`&`):** `magnitude` needs to read the vector components
  but not modify them, so it takes a shared reference (`&[f64; 3]`).
- **Mutable References (`&mut`):** `normalize` modifies the vector in place, so
  it requires an exclusive mutable reference (`&mut [f64; 3]`).
- **Dereferencing:** In the `normalize` loop, `item` is a `&mut f64`. To modify
  the actual value, we must dereference it using `*item`.
- **Iteration:** When we iterate over a reference to an array (like `vector`),
  the iterator yields references to the elements. In `magnitude`, `coord` is
  `&f64`. In `normalize`, `item` is `&mut f64`.

<details>

- Note that in `normalize` we were able to do `*item /= mag` to modify each
  element. This is because we're iterating using a mutable reference to an
  array, which causes the `for` loop to give mutable references to each element.

- It is also possible to take slice references here, e.g.,
  `fn
  magnitude(vector: &[f64]) -> f64`. This makes the function more general,
  at the cost of a runtime length check.

</details>
