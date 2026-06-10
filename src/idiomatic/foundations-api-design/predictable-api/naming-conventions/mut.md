---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `[method]_mut`: Mutable reference access

Suffix for access-style methods.

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> Vec<T> {
    fn get(&self, index: usize) -> Option<&T>;
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
}

impl<T> [T] {
    fn iter(&self) -> impl Iterator<Item = &T>;
    fn iter_mut(&mut self) -> impl Iterator<Item = &mut T>;
}
```

<details>

- Suffix that signifies the method gives access to a mutable reference.

- Requires mutable access to the value you're calling this method on.

- Rust can't abstract over mutability, so there's no way to write a method that
  can be used both mutably and immutably. Instead, we write pairs of functions,
  where the immutable version gets the shorter name and the mutable version gets
  the `_mut` suffix.

</details>
