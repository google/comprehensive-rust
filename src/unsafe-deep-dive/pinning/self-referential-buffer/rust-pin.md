---
minutes: 10
---

# With Pin&lt;Ptr&gt;

Pinning allows Rust programmers to create a type which is much more similar to
the C++ class.

```rust,editible
use std::marker::PhantomPinned;
use std::pin::Pin;

/// A self-referential buffer that cannot be moved.
#[derive(Debug)]
pub struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *mut u8,
    _pin: PhantomPinned,
}

impl SelfReferentialBuffer {
    pub fn new() -> Pin<Box<Self>> {
        let buffer = SelfReferentialBuffer {
            data: [0; 1024],
            cursor: std::ptr::null_mut(),
            _pin: PhantomPinned,
        };
        let mut pinned = Box::pin(buffer);

        unsafe {
            let mut_ref = Pin::get_unchecked_mut(pinned.as_mut());
            mut_ref.cursor = mut_ref.data.as_mut_ptr();
        }

        pinned
    }

    pub fn read(&self, n_bytes: usize) -> &[u8] {
        unsafe {
            let start = self.data.as_ptr();
            let end = start.add(self.data.len());
            let cursor = self.cursor as *const u8;

            assert!((start..=end).contains(&cursor), "cursor is out of bounds");

            let offset = cursor.offset_from(start) as usize;
            let available = self.data.len().saturating_sub(offset);
            let len = n_bytes.min(available);

            &self.data[offset..offset + len]
        }
    }

    pub fn write(mut self: Pin<&mut Self>, bytes: &[u8]) {
        let this = unsafe { self.as_mut().get_unchecked_mut() };
        unsafe {
            let start = this.data.as_mut_ptr();
            let end = start.add(1024);

            assert!((start..=end).contains(&this.cursor), "cursor is out of bounds");
            let available = end.offset_from(this.cursor) as usize;
            let len = bytes.len().min(available);

            std::ptr::copy_nonoverlapping(bytes.as_ptr(), this.cursor, len);
            this.cursor = this.cursor.add(len);
        }
    }
}
```

<details>

Note that the function signatures have now changed. For example, `::new()`
returns `Pin<Box<Self>>` rather than `Self`. This incurs a memory allocation
because `Pin<Ptr>` must work with a pointer type.

Indirection between `get_mut()` and `get_unchecked_mut()` is necessary because
`self.get_unchecked_mut()` receives `self` by value and the `pinned` variable is
a `Box`. The `Box` does not provide its inner value via its `Deref`
implementation, which provides the normal auto-deref behavior. `get_mut()`
returns `Pin<&mut SelfReferentialBuffer>`, which resolves the issue.

</details>
