---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `into`

Prefix for methods that convert `self` into another type. Consumes `self`,
returns an owned value.

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub trait IntoIterator {
    fn into_iter(self) -> Self::IntoIter;
}

impl str {
    fn into_string(self: Box<str>) -> String;
}
```

<details>

- Prefix for a function that consumes an owned value and transforms it into a
  value of another type.

- Not reinterpret cast! The data can be rearranged, reallocated, changed in any
  way, including losing information.

- `into_iter` consumes a collection (like a vec, or a btreeset, or a hashmap)
  and produces an iterator over owned values, unlike `iter` and `iter_mut` which
  produce iterators over reference values.

</details>
