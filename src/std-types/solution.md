<!--
Copyright 2023 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2023 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

- **Generic Key Type:** `Counter` is now generic over `T`.
- **Trait Bounds:** `T` must be `Eq + Hash` to be a key in a `HashMap`. We apply
  these bounds on the `impl` block so that `HashMap<T, u64>` works.
- **Entry API:** `self.values.entry(value).or_default()` provides a mutable
  reference to the value for the key. If the key is missing, it inserts the
  default value for `u64` (which is `0`) and returns a reference to that. This
  is concise and efficient (only one hash lookup).

<details>

- Explain that `Eq` implies `PartialEq`. `HashMap` requires full `Eq` to handle
  keys correctly.
- `or_default()` works because `u64` implements `Default`.
- `unwrap_or_default()` works similarly for `Option`: if the key isn't found
  (`None`), it returns `0`.

</details>
