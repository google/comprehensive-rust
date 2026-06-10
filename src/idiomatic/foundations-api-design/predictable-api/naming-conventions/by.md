---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `by`: custom comparator or projection

Component for methods that take a custom projection or comparison function.

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> [T] {
    fn sort(&mut self) where T: Ord;

    fn sort_by(&mut self, compare: impl FnMut(&T, &T) -> Ordering);

    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
}
```

<details>

- `sort_by` takes a custom comparator function that replaces the normal `Ord`
  comparison logic.

- `sort_by_key` takes a projection function that takes the original element and
  returns an alternate value to use for sorting. This allow us to do things like
  sort by a particular field of a struct.

- Sometimes the "by" preposition is simply a preposition:

  - [`Read::by_ref()`](https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref)

  - [`Iterator::advance_by()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.advance_by)
    iterator method (nightly feature)

</details>
