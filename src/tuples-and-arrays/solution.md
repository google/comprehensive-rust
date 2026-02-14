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

- **Array Types:** The type `[[i32; 3]; 3]` represents an array of size 3, where
  each element is itself an array of 3 `i32`s. This is how multi-dimensional
  arrays are typically represented in Rust.
- **Initialization:** We initialize `result` with zeros (`[[0; 3]; 3]`) before
  filling it. Rust requires all variables to be initialized before use; there is
  no concept of "uninitialized memory" in safe Rust.
- **Copy Semantics:** Arrays of `Copy` types (like `i32`) are themselves `Copy`.
  When we pass `matrix` to the function, it is copied by value. The `result`
  variable is a new, separate array.
- **Iteration:** We use standard `for` loops with ranges (`0..3`) to iterate
  over indices. Rust also has powerful iterators, which we will see later, but
  indexing is straightforward for this matrix transposition.

<details>

- Mention that `[i32; 3]` is a distinct type from `[i32; 4]`. Array sizes are
  part of the type signature.
- Ask students what would happen if they tried to return `matrix` directly after
  modifying it (if they changed the signature to `mut matrix`). (Answer: It
  would work, but it would return a modified _copy_, leaving the original in
  `main` unchanged).

</details>
