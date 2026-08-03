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

The solution addresses several bugs in the original implementation:

- **Input Validation:** Whitespace is now handled explicitly, and invalid
  characters trigger an immediate failure rather than being silently ignored.
- **Minimum Length:** The code now correctly enforces the requirement for at
  least two digits.
- **Test Coverage:** Added tests for edge cases like empty strings, single
  digits, and invalid characters.

<details>

- **Unicode Safety:** Rust strings are UTF-8, so they cannot be indexed by
  integer. Using `.chars().rev()` is the idiomatic way to process a string
  backwards while correctly handling multi-byte characters.
- **Handling `Option`:** Using `if let Some(digit) = c.to_digit(10)` is the
  standard way to handle checked conversions, providing a safe way to deal with
  potential conversion failures without relying on sentinel values.
- **Iterators vs. Loops:** This algorithm could be expressed as a functional
  iterator chain using `filter_map`, `enumerate`, and `fold`. However, an
  imperative loop is often more readable when maintaining multiple pieces of
  interdependent state like `sum`, `double`, and `digits`.

</details>
