# Implementing Unsafe Traits

Like with functions, you can mark a trait as `unsafe` if the implementation must guarantee
particular conditions to avoid undefined behaviour.

For example, the `zerocopy` crate has an unsafe trait that looks
[something like this](https://docs.rs/zerocopy/latest/zerocopy/trait.AsBytes.html):

```rust,editable
use std::mem::size_of_val;
use std::slice;

/// ...
/// # Safety
/// The type must have a defined representation and no padding.
pub unsafe trait AsBytes {
    fn as_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(self as *const Self as *const u8, size_of_val(self))
        }
    }
}

// Safe because u32 has a defined representation and no padding.
unsafe impl AsBytes for u32 {}
```

<details>

There should be a `# Safety` section on the Rustdoc for the trait explaining the requirements for
the trait to be safely implemented.

The actual safety section for `AsBytes` is rather longer and more complicated.

The built-in `Send` and `Sync` traits are unsafe.

</details>
