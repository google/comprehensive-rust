---
minutes: 15
---

# Case Study: RawVec

> WORK IN PROGRESS
>
> This section is likely to receive significant alterations before completion
> and may even be removed entirely.

Many important collections in the standard library, such as `Vec<T>`, `String`
and `Deque<T>` rely on a private inner type called `RawVec<T>`.

Why is that inner type used?

```rust,ignore
// https://doc.rust-lang.org/src/alloc/vec/mod.rs.html
// std::alloc
pub struct Vec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}
```

```rust,ignore
// std::raw_vec
pub(crate) struct RawVec<T, A: Allocator = Global> {
    inner: RawVecInner<A>,
    _marker: PhantomData<T>,
}

struct RawVecInner<A: Allocator = Global> {
    ptr: Unique<u8>,
    /// Never used for ZSTs; it's `capacity()`'s responsibility to return usize::MAX in that case.
    ///
    /// # Safety
    ///s
    /// `cap` must be in the `0..=isize::MAX` range.
    cap: Cap,
    alloc: A,
}
```

The [implementation of `RawVec` is described in the Rustonomicon][rv].

[rv]: https://doc.rust-lang.org/nomicon/vec/vec-raw.html

<details>

`Vec<T>` is normally described as being a struct with three fields: length,
capacity, and pointer to an underlying buffer. Once you dig into the
implementation details, you'll notice that things are much more complicated.

`RawVec<T>` provides a barrier between Safe and Unsafe.

`RawVec<T>`

`RawVecInner<A>` contains the actual pointer and capacity of the underlying
buffer.

</details>
