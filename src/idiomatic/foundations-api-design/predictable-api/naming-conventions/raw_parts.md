---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `raw_parts`

Peeling back safe abstractions on heap data.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl<T> Vec<T> {
    // Note how this is an unsafe function
    unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Vec<T>;

    fn into_raw_parts(self) -> (*mut T, usize, usize);
}
```

<details>

- `raw_parts` denotes methods that construct items from or decompose items into
  underlying pointer data and its relevant layout information (capacity, etc.).

- These kinds of methods can be marked as `unsafe` if constructing new values as
  trust is placed on the user to avoid conditions that might lead to undefined
  behavior.

  Such a case might be passing a pointer of `sizeof T * 10` to
  `Vec::from_raw_parts` but also passing `20` as the capacity argument, which
  would lead to writing or accessing values 10 through 19 in the vector being
  undefined behavior.

</details>
