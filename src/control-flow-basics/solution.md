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

This solution demonstrates a few key Rust features:

- **`mut` arguments:** The `n` argument is declared as `mut n`. This makes the
  local variable `n` mutable within the function scope. It does _not_ affect the
  caller's value, as integers are `Copy` types passed by value.
- **`if` expressions:** Rust's `if` is an expression, meaning it produces a
  value. We assign the result of the `if`/`else` block directly to `n`. This is
  more concise than writing `n = ...` inside each branch.
- **Implicit return:** The function ends with `len` (without a semicolon), which
  is automatically returned.

<details>

- Note that `n` must be strictly greater than 0 for the Collatz sequence to be
  valid. The function signature takes `i32`, but the problem description implies
  positive integers. A more robust implementation might use `u32` or return an
  `Option` or `Result` to handle invalid inputs (0 or negative numbers), but
  panic or infinite loops are potential outcomes here if `n <= 0`.
- The overflow is a potential issue if `n` grows too large, similar to the
  Fibonacci exercise.

</details>
