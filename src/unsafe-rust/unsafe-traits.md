---
minutes: 5
---

# Implementing Unsafe Traits

Like with functions, you can mark a trait as `unsafe` if the implementation must
guarantee particular conditions to avoid undefined behaviour.

For example, the `zerocopy` crate has an unsafe trait that looks
[something like this](https://docs.rs/zerocopy/latest/zerocopy/trait.IntoBytes.html):

```rust,editable
use std::mem;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait IntoBytes {
    fn as_bytes(&self) -> &[u8] 
    where
        Self: Immutable
    {
        let len = mem::size_of_val(self);
        let slf: *const Self = self;
        unsafe {
            slice::from_raw_parts(slf.cast::<u8>(), len)
        }
    }
}
```

<details>

There should be a `# Safety` section on the Rustdoc for the trait explaining the
requirements for the trait to be safely implemented.

The actual safety section for `IntoBytes` is rather longer and more complicated.

The built-in `Send` and `Sync` traits are unsafe.

</details>
