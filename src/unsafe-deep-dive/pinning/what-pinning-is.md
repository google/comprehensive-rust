# What pinning is

Abridged `Pin` from the Rust standard library:

```rust,ignore
#[repr(transparent)]
pub struct Pin<Ptr> {
    pointer: Ptr,
}

impl<Ptr: Deref<Target: Unpin>> Pin<Ptr> {
    pub fn new(pointer: Ptr) -> Pin<Ptr> { ... }

    pub fn into_inner(pin: Pin<Ptr>) -> Ptr { ... }

    pub unsafe fn new_unchecked(pointer: P) -> Pin<Ptr> { ... }
}
```

<details>

Conceptually, pinning prevents the default movement behavior.

This appears to be a change in the language itself.

However, the `Pin` wrapper doesn't actually change anything fundamental about
the language.

`Pin` doesn't expose safe APIs that would allow a move. Thus, it can prevent
bitwise copy.

Unsafe APIs allow library authors to wrap types that do not implement `Unpin`,
but they must uphold the same guarantees.

The documentation of `Pin` uses the term "pointer types".

The term "pointer type" is much more broad than the pointer primitive type in
the language.

A "pointer type" wraps every type that implements `Deref` with a target that
implements `Unpin`.

Rust style note: This trait bound is enforced through trait bounds on the
`::new()` constructor, rather than on the type itself.

</details>
