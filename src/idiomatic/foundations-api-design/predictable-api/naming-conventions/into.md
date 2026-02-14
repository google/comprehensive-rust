---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `into`

- Prefix for methods that convert `self` into another type.

Consumes `self`, returns an owned value.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> Vec<T> {
    fn into_parts(self) -> (NonNull<T>, usize, usize);
}

impl<T> Cell<T> {
    fn into_inner(self) -> T;
}
```

<details>
- Prefix for a function that consumes an owned value and transforms it into a value of another type.

Not reinterpret cast! The data can be rearranged, reallocated, changed in any
way, including losing information.

- corollary to `From`

- `into_iter` consumes a collection (like a vec, or a btreeset, or a hashmap)
  and produces an iterator over owned values, unlike `iter` and `iter_mut` which
  produce iterators over reference values.

- Ask the class: what will `Vec::into_raw_parts` do?

</details>
