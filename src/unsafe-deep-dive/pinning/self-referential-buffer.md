---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Self-Referential Buffer Example

A "self-referential buffer" is a type that has a reference to one of its own
fields:

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *mut u8,
}
```

This kind of structure is not typical in Rust, because there's no way to update
the cursor's address when instances of `SelfReferentialBuffer` move.

However, this kind of setup is more natural in other languages that provide
garbage collection, and also C++ that allows users to define their own behavior
during moves and copies.

## Outline

{{%segment outline}}
