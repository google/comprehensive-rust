---
minutes: 5
---

# Implementing Unsafe Traits

Like with functions, you can mark a trait as `unsafe` if the implementation must
guarantee particular conditions to avoid undefined behaviour.

For example, the `zerocopy` crate has an unsafe trait that looks
[something like this](https://docs.rs/zerocopy/latest/zerocopy/trait.IntoBytes.html):

```rust,editable
use std::{mem, slice};

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait IntoBytes {
    fn as_bytes(&self) -> &[u8] {
        let len = mem::size_of_val(self);
        unsafe { slice::from_raw_parts((&raw const self).cast::<u8>(), len) }
    }
}

// SAFETY: `u32` has a defined representation and no padding.
unsafe impl IntoBytes for u32 {}
```

<details>

There should be a `# Safety` section on the Rustdoc for the trait explaining the
requirements for the trait to be safely implemented.

The actual safety section for `IntoBytes` is rather longer and more complicated.

The built-in `Send` and `Sync` traits are unsafe.

</details>
