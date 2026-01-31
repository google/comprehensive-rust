# Solution

```rust,editable
{{#include exercise.rs:solution}}
```

- **`cycle()`:** We create an iterator `b` that repeats the elements of `values`
  infinitely. This handles the wraparound requirement elegantly without manual
  index modulo arithmetic.
- **`skip()`:** We advance the cycled iterator by `offset` positions.
- **`zip()`:** We combine the original iterator `a` with the offset iterator
  `b`. The `zip` function yields pairs of elements. Crucially, `zip` stops as
  soon as either iterator is exhausted. Since `a` has the length of `values` and
  `b` is infinite, the resulting zipped iterator has the same length as
  `values`.
- **`map()` and `collect()`:** We calculate the difference for each pair and
  collect the results into a new `Vec`.

<details>

- Mention that `values.iter()` creates an iterator that yields references
  (`&i32`). In the `map` closure, we dereference them (`*b - *a`) to perform
  arithmetic.
- This functional approach is often more concise and less error-prone (no
  off-by-one index errors) than manual looping with indices.

</details>
