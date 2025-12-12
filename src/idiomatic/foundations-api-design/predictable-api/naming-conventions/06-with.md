---
minutes: 5
---

# `with`

Prefix for various setter and constructor style functions.

```rust
impl<T> Vec<T> {
    // Constructor style.
    fn with_capacity(capacity: usize) -> Vec<T>;
}

impl Path {
    // Simplified. Clone-And-Set style.
    fn with_extension(&self, ext: &OsStr) -> Path;
}
```

<details>
- Prefix for methods that create a new copy of a data structure.

Can be constructors, builders, or setters.

- Constructor-style `with` methods usually set one specific field but leave
  everything else "default"

  `with_capacity` allocates enough space for the number of elements given, but
  does not otherwise add anything to the data structure.

- When `with` methods take an owned value, they're builder-style.

- When `with` methods take a reference, like pathbuf's `with` methods, they
  return a new owned value while the original value remains.

</details>
