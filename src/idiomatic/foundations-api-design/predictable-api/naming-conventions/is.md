---
minutes: 2
---

<!--
Copyright 2025 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# `is_[condition]`: Boolean Check

Check a condition about a datatype.

```rust,compile_fail
# // Copyright 2025 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
impl <T> Vec<T> {
    is_empty(&self) -> bool;
}

impl f32 {
    is_nan(self) -> bool;
}

impl u32 {
    is_power_of_two(self) -> bool;
}
```

<details>
- A boolean condition on a value.

- `is` prefix is preferred over methods with `not` in the name.

  There are no instances of `is_not_` in standard library methods, just use
  `!value.is_[condition]`.

</details>
