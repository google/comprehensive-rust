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

The solution demonstrates the fundamental distinction between shared and
exclusive references:

- **Shared References (`&`):** Used in `magnitude` because the function only
  reads the vector components.
- **Exclusive References (`&mut`):** Required in `normalize` to modify the array
  elements in place.
- **Explicit Dereferencing:** Inside `normalize`, `item` is an `&mut f64`. We
  use `*item` to access and modify the underlying value.

<details>

- **Iterating over References:** Iterating over `&vector` or `&mut vector`
  yields references to the elements. This is why `coord` is `&f64` and `item` is
  `&mut f64`.
- **Arrays vs. Slices:** The functions are defined using array references
  (`&[f64; 3]`), which ensures the length is known at compile time. Using slices
  (`&[f64]`) would make the functions more flexible but would introduce a
  runtime length check or potential for panics if the slice has the wrong size.
- **Method Call Ergonomics:** In `magnitude`, we can call `mag_squared.sqrt()`
  directly. In `normalize`, we pass `vector` (an `&mut [f64; 3]`) to
  `magnitude`, and Rust automatically downgrades the exclusive reference to a
  shared reference to match the signature.

</details>
