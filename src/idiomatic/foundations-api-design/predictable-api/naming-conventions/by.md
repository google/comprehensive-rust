---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `by`: custom comparator or projection

Component for methods that take a custom projection or comparison function.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> [T] {
    // Simplified
    fn sort_by(&mut self, compare: impl FnMut(&T, &T) -> Ordering);

    // Uses a predicate to determine what items end up in non-overlapping chunks.
    fn chunk_by_mut<F: FnMut(&T, &T) -> bool>(
        &mut self,
        pred: F,
    ) -> ChunkByMut<'_, T, F>;
}

trait Iterator {
    // Provided method of Iterator. Simplified.
    fn min_by<F>(
        self,
        compare: impl FnMut(&Self::Item, &Self::Item) -> Ordering,
    ) -> Option<Self::Item>;
}
```

<details>
- Method will take a comparison or projection function.

A projection function here being a function that, given a reference to a value
that exists in the data structure, will compute a value to perform the principle
computation with.

Methods like `sort_by_key` allow us to sort by _the hash function I've passed to
the method_ or sort by _this specific field of the data in the slice_.

For example, if you have a slice of values of some data structure you might want
to sort them by a field of that data structure, or even a hash value of that
data.

`sort_by` takes a comparator function directly.

- Most often seen in methods that sort or otherwise manipulate a slice with a
  custom sort or comparison function rather than by the `Ord` implementation of
  the type itself.

- Sometimes the "by" preposition is simply a preposition.

  "by", like some other name components, may end up in a method name for normal
  linguistic reasons rather than holding specific naming convention semantic
  weight.

  - [`Read::by_ref()`](https://doc.rust-lang.org/std/io/trait.Read.html#method.by_ref)

  - [`Iterator::advance_by()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.advance_by)
    iterator method (nightly feature)

</details>
