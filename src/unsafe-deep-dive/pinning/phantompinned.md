---
minutes: 5
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# PhantomPinned

## Definition

```rust,ignore
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct PhantomPinned;

impl !Unpin for PhantomPinned {}
```

## Usage

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
pub struct DynamicBuffer {
    data: Vec<u8>,
    cursor: std::ptr::NonNull<u8>,
    _pin: std::marker::PhantomPinned,
}
```

<details>

`PhantomPinned` is a marker type.

If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.

This has the effect of enforcing pinning when `DynamicBuffer` is wrapped by
`Pin`.

</details>

<!-- TODO: Monitor issue https://github.com/rust-lang/rust/issues/125735 as this guidance will change at some point and future code will move to UnsafePinned -->
