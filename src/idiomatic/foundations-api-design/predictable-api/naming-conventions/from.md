---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `from`

A constructor function, strongly implying "type conversion".

```rust,compile_fail,editable
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl Duration {
    fn from_days(days: u64) -> Duration;
}

impl i32 {
    fn from_ascii(src: &[u8]) -> Result<i32, ParseIntError>;
}

impl u32 {
    fn from_le_bytes(bytes: [u8; 4]) -> u32;
}
```

<details>

- Prefix for constructor-style, `From`-trait-style functions.

- These functions can take multiple arguments, but usually imply the user is
  doing more of the work than a usual constructor would.

- `new` is still preferred for most constructor-style functions, the implication
  for `from` is transformation of one data type to another.

</details>
