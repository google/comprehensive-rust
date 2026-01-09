---
minutes: 5
---

# Definition of Pin

```rust,ignore
#[repr(transparent)]
pub struct Pin<Ptr> {
    pointer: Ptr,
}

impl<Ptr: Deref<Target: Unpin>> Pin<Ptr> {
    pub fn new(pointer: Ptr) -> Pin<Ptr> { ... }
}

impl<Ptr: Deref> Pin<Ptr> {
    pub unsafe fn new_unchecked(pointer: Ptr) -> Pin<Ptr> { ... }
}
```

<details>

`Pin` is a minimal wrapper around a _pointer type_, which is defined as a type
that implements `Deref`.

However, `Pin::new()` only accepts types that dereference into a target that
implements `Unpin` (`Deref<Target: Unpin>`). This allows `Pin` to rely on the
the type system to enforce its guarantees.

Types that do not implement `Unpin`, i.e. types that require pinning, must
create a `Pin` via the unsafe `Pin::new_unchecked()`.

Aside: Unlike other `new()`/`new_unchecked()` method pairs, `new` does not do
any runtime checking. The check is a zero-cost compile-time check.

</details>
