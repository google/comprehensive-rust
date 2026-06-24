<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
{{#include exercise.rs:solution}}
```

The solution demonstrates zero-copy parsing, a high-performance pattern made
safe by Rust's lifetime system:

- **Zero-Copy Parsing:** The `Person` and `PhoneNumber` structs do not own their
  strings; instead, they borrow them directly from the input byte array. No
  allocations are performed during parsing.
- **Enforced Safety:** The lifetime parameter `'a` ties the output structures to
  the input buffer. The compiler ensures that a `Person` cannot outlive the
  bytes it was parsed from, preventing use-after-free bugs.
- **Efficient Slicing:** Using `split_at` allows for efficient traversal of the
  input buffer by manipulating pointers and lengths without copying data.

<details>

- **Lifetime Propagation:** Note how the lifetime `'a` is propagated through the
  recursive `parse_message` and `add_field` calls. This creates a chain of
  borrows that the borrow checker tracks throughout the entire execution.
- **Trade-offs:** While zero-copy parsing is extremely fast, it requires the
  original input buffer to remain in memory as long as the parsed data is in
  use. In some applications, copying into owned `String`s might be preferable to
  free up the input buffer sooner.

</details>
