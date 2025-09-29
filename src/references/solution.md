# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

<details>

- Note that in `normalize` we were able to do `*item /= mag` to modify each
  element. This is because we're iterating using a mutable reference to an
  array, which causes the `for` loop to give mutable references to each element.

- It is also possible to take slice references here, e.g.,
  `fn
  magnitude(vector: &[f64]) -> f64`. This makes the function more general,
  at the cost of a runtime length check.

</details>
