# Tokio's Intrusive Linked List

> Current as of tokio v1.48.0

The Tokio project maintains an [intrusive linked list implementation][ill] that
demonstrates many use cases of `unsafe` and a number of types and traits from
Rust's unsafe ecosystem, including `cell::UnsafeCell`, `mem::ManuallyDrop`,
[pinning](../pinning/what-pinning-is.md), and unsafe traits.

A linked list is a difficult data structure to implement in Rust, because it
relies heavily on stable memory addresses remaining stable. This isn't something
that happens naturally, as Rust types change their memory address every time
they move.

## Introductory Walkthrough

The public API is provided by the `LinkedList<L, T>` type, which contains fields
for the start and the end of the list. `Option<NonNull<T>>` could be read as a
`Option<*mut T>`, with the assurance that null pointers will never be created.

`NonNull<T>` is "_covariant_ over `T`", which means that `NonNull<T>` inherits
the [variance] relationships from `T`.

```rust,ignore
use core::marker::PhantomData;

// ...

/// An intrusive linked list.
///
/// Currently, the list is not emptied on drop. It is the caller's
/// responsibility to ensure the list is empty before dropping it.
pub(crate) struct LinkedList<L, T> {
    /// Linked list head
    head: Option<NonNull<T>>,

    /// Linked list tail
    tail: Option<NonNull<T>>,

    /// Node type marker.
    _marker: PhantomData<*const L>,
}
```

`LinkedList` is neither `Send` nor `Sync`, unless its targets are.

```rust,ignore
unsafe impl<L: Link> Send for LinkedList<L, L::Target> where L::Target: Send {}
unsafe impl<L: Link> Sync for LinkedList<L, L::Target> where L::Target: Sync {}
```

The `Link` trait used the those trait bounds is defined next. `Link` is an
unsafe trait that manages the relationships between nodes when the list needs to
pass references externally to callers.

Here's trait's definition. The most significant method is `pointers()`, which
returns a `Pointers` struct. `Pointers` provides access to the two ends of the
link by marking itself as `!Unpin`.

```rust,ignore
pub unsafe trait Link {
    type Handle;

    type Target;

    fn as_raw(handle: &Self::Handle) -> NonNull<Self::Target>;

    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle;

    /// # Safety
    ///
    /// The resulting pointer should have the same tag in the stacked-borrows
    /// stack as the argument. In particular, the method may not create an
    /// intermediate reference in the process of creating the resulting raw
    /// pointer.
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<Pointers<Self::Target>>;
}
```

`Pointers` is where the magic happens:

```rust,ignore
pub(crate) struct Pointers<T> {
    inner: UnsafeCell<PointersInner<T>>,
}

struct PointersInner<T> {
    prev: Option<NonNull<T>>,
    next: Option<NonNull<T>>,

    /// This type is !Unpin due to the heuristic from:
    /// <https://github.com/rust-lang/rust/pull/82834>
    _pin: PhantomPinned,
}
```

## Remarks

Understanding the whole implementation will take some time, but it's a rewarding
experience. The code demonstrates composing many parts of unsafe Rust's
ecosystem into a workable, high performance data structure. Enjoy exploring!

[ill]: https://docs.rs/tokio/1.48.0/src/tokio/util/linked_list.rs.html
[variance]: https://doc.rust-lang.org/reference/subtyping.html
