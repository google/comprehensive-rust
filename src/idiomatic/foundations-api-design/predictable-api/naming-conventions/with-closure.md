---
minutes: 2
---

# `with`: Working with Closures

`with` as in "do X, but with this specific way of computing things."

```rust,compile_fail
impl<T> Vec<T> {
    // Simplified. If the resize is larger than the current vec size, use the
    // closure to populate elements.
    pub fn resize_with(&mut self, new_len: usize, f: impl FnMut() -> T);
}

mod iter {
    // Create an infinite, lazy iterator using a closure.
    pub fn repeat_with<A, F: FnMut() -> A>(repeater: F) -> RepeatWith<F>;
}
```

<details>

- `with` can appear as a suffix to communicate there is a specific function or
  closure that can be used instead of a "sensible default" for a computation.

  Similar to [`by`](./by.md).

</details>
