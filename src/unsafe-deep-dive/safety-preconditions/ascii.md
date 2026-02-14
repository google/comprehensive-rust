---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Example: ASCII Type

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
/// Text that is guaranteed to be encoded within 7-bit ASCII.
pub struct Ascii<'a>(&'a mut [u8]);

impl<'a> Ascii<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Option<Self> {
        bytes.iter().all(|&b| b.is_ascii()).then(|| Ascii(bytes))
    }

    /// Creates a new `Ascii` from a byte slice without checking for ASCII
    /// validity.
    ///
    /// # Safety
    ///
    /// Providing non-ASCII bytes results in undefined behavior.
    pub unsafe fn new_unchecked(bytes: &'a mut [u8]) -> Self {
        Ascii(bytes)
    }
}
```

<details>

"The `Ascii` type is a minimal wrapper around a byte slice. Internally, they
share the same representation. However, `Ascii` requires that the high bit must
not be used."

Optional: Extend the example to mention that it's possible to use
`debug_assert!` to test the preconditions during tests without impacting release
builds.

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
unsafe fn new_unchecked(bytes: &mut [u8]) -> Self {
    debug_assert!(bytes.iter().all(|&b| b.is_ascii()))
    Ascii(bytes)
}
```

</details>
