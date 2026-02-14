---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `try_[method]`: Fallible methods with Specific Errors

Prefix for fallible methods that return a `Result`.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl TryFrom<i32> for u32 {
    type Error = TryFromIntError;
    fn try_from(value: i32) -> Result<i64, TryFromIntError>;
}

impl<T> Receiver<T> {
    try_recv(&self) -> Result<T, TryRecvError>;
}
```

<details>
- Prefix for methods that can fail, returning a `Result`.

- `TryFrom` is a `From`-like trait for types whose single-value constructors
  might fail in some way.

- Ask: Why aren't `Vec::get` and other similar methods called `try_get`?

  Methods are named `get` if they return a reference to an existing value and
  return an `Option` instead of `Result` because there is only one failure mode.
  For example, only "index out of bounds" for `Vec::get`, and "key does not
  exist" for `HashMap::get`.

</details>
