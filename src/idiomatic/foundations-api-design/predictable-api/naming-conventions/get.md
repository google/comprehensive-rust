---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `get`: Borrow an Element

Getting an element from a collection or container.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl <T> Vec<T> {
    fn get(&self, index: usize) -> Option<&T> {...}
}

impl <T> OnceCell {
    fn get(&self) -> Option<&T> {...}
}
```

<details>
- Gets are trivial, they get a value!

Immutable by default.

Should not panic. May return an option or result, depending on the framework.

- Not for fields!

  For private fields you don't want users to have direct, assign a method with a
  more descriptive name (or the same name as the field) is preferred.

</details>
