---
minutes: 2
---

# `push`

Common on array-like structures.

```rust
impl<T> Vec<T> {
  fn push(&mut self, value: T)
}

impl<T> VecDeque<T> {
  fn push_back(&mut self, value: T)
  fn push_front(&mut self, value: T)
}
```

<details>
- Modifies a sequential collection by adding an element.

- Takes `self` by mutable reference.
