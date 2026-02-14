---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `push`

Common on array-like structures.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> Vec<T> {
    fn push(&mut self, value: T);
}

impl<T> VecDeque<T> {
    fn push_back(&mut self, value: T);
    fn push_front(&mut self, value: T);
}
```

<details>
- Modifies a sequential collection by adding an element.

- Takes `self` by mutable reference.
