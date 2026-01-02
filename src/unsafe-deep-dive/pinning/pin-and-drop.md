---
minutes: 10
---

# <code>Pin&lt;Ptr&gt;</code> and <code>Drop</code>

A key challenge with pinned, `!Unpin` types is implementing the `Drop` trait.
The `drop` method takes `&mut self`, which allows moving the value. However,
pinned values must not be moved.

## An Incorrect `Drop` Implementation

It's easy to accidentally move a value inside `drop`. Operations like assignment,
`ptr::read`, and `mem::replace` can silently break the pinning guarantee.

```rust,editable
struct SelfRef {
    data: String,
    ptr: *const String,
}

impl Drop for SelfRef {
    fn drop(&mut self) {
        // BAD: `ptr::read` moves `self.data` out of `self`.
        // When `_dupe` is dropped at the end of the function, it's a double free!
        let _dupe = unsafe { std::ptr::read(&self.data) };
    }
}
```

<details>
`!Unpin` types can make it difficult to safely implement `Drop`. Implementations
must not move pinned values.

Pinned types make guarantees about memory stability. Operations like `ptr::read`
and `mem::replace` can silently break these guarantees by moving or duplicating
data, invalidating internal pointers without the type system's knowledge.

In this `drop()` method, `_dupe` is a bitwise copy of `self.data`. At the end of
the method, it will be dropped along with `self`. This double drop is undefined
behavior.
</details>

## A Correct `Drop` Implementation

To implement `Drop` correctly for a `!Unpin` type, you must ensure that the
value is not moved. A common pattern is to create a helper function that operates
on `Pin<&mut T>`.

```rust,editable
use std::{marker::PhantomPinned, pin::Pin};

struct SelfRef {
    data: String,
    ptr: *const String,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(data: impl Into<String>) -> Pin<Box<SelfRef>> {
        let mut this = Box::pin(SelfRef {
            data: data.into(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        let ptr: *const String = &this.data;
        // SAFETY: `this` is pinned before we create the self-reference.
        unsafe {
            Pin::as_mut(&mut this).get_unchecked_mut().ptr = ptr;
        }
        this
    }

    // This function can only be called on a pinned `SelfRef`.
    unsafe fn drop_pinned(self: Pin<&mut SelfRef>) {
        // `self` is pinned, so we must not move out of it.
        println!("dropping {}", self.data);
    }
}

impl Drop for SelfRef {
    fn drop(&mut self) {
        // We can safely call `drop_pinned` because `drop` is the last time
        // the value is used. We use `new_unchecked` because we know `self`
        // will not be moved again.
        unsafe {
            SelfRef::drop_pinned(Pin::new_unchecked(self));
        }
    }
}

fn main() {
    let _pinned = SelfRef::new("Hello, ");
} // `Drop` runs without moving the pinned value
```
