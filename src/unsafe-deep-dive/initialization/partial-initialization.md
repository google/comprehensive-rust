<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Partial Initialization

```rust
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
use std::mem::MaybeUninit;

fn main() {
    // let mut buf = [0u8; 2048];
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

    let external_data = b"Hello, Rust!";
    let len = external_data.len();

    for (dest, src) in buf.iter_mut().zip(external_data) {
        dest.write(*src);
    }

    // SAFETY: We initialized exactly 'len' bytes of `buf` with UTF-8 text
    let text: &str = unsafe {
        let ptr: *const u8 = buf.as_ptr().cast::<u8>();
        let init: &[u8] = std::slice::from_raw_parts(ptr, len);
        std::str::from_utf8_unchecked(init)
    };

    println!("{text}");
}
```

<details>

This code simulates receiving data from some external source.

When reading bytes from an external source into a buffer, you typically don't
know how many bytes you'll receive. Using `MaybeUninit<T>` lets you allocate the
buffer once without paying for a redundant initialization pass.

If we were to create the array with the standard syntax (`buf = [0u8; 2048]`),
the whole buffer would be flushed with zeroes. `MaybeUninit<T>` tells the
compiler to reserve space, but not to touch the memory yet.

Q: Which part of the code snippet is performing a similar role to
`.assume_init()`? A: The pointer cast and the implicit read.

We cannot call `assume_init()` on the whole array. That would be unsound because
most elements remain uninitialized. Instead, we cast the pointer from
`*const MaybeUninit<u8>` to `*const u8` and build a slice covering only the
initialised portion.

</details>
