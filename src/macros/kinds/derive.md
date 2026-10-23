---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Derive Macros

**Derive macros** are attached to structs, enums, or unions by preceding them
with the `#[derive(...)]` attribute. They generate new code next to the type,
typically implementing traits automatically.

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#

#[derive(Debug, Default, Clone)]
enum TestResult {
    Waived,
    #[default]
    NotYetTaken,
    Graded(u32),
}

fn main() {
    let test_result = TestResult::default();
    println!("Default result: {:?}", test_result);
}
```

- **Targeted:** Can only be applied to type definitions using the `struct`,
  `enum`, or `union` keywords.
- **Additive:** They cannot modify or delete the original type definition; they
  can only output additional code (such as `impl` blocks).
- **Implementation:** Always implemented as procedural macros.

<details>

- Explain that `#[derive(Debug)]` generates an implementation of
  `std::fmt::Debug` for the struct.
- Note that derive macros can also support "helper attributes" (attributes
  placed on struct fields, like `#[serde(rename = "name")]` or `#[default]` in
  the example), which configure the generated code.
- Mention that derive macros are compiled together in the same module as the
  type. Any items they generate (like helper functions or helper traits) exist
  in the same namespace, so they must be designed carefully to avoid namespace
  collisions.

</details>
