---
minutes: 5
---

# `unchecked`: Unsafe

`unchecked` distinguishes the unsafe function in a safe/unsafe pair.

Don't add "unchecked" to the name of every unsafe function.

```rust,no_compile
impl <T> NonNull<T> {
    // A checked version of the constructor, `None` on null.
    fn new(ptr: *mut T) -> Option<NonNull<T>>

    // Unchecked cosntructor, you can violate the non-null invariant!
    unsafe fn new_unchecked(ptr: *mut T) -> NonNull<T>
}

impl <T> Vec<T> {
    // Panics on OOB, old API design.
    fn split_at(&self, mid: usize) -> (&[T], &[T])

    // Newer method, returns `None` if mid > len
    fn split_at_checked(&self, mid: usize) -> Option<(&[T], &[T])>

    // Unchecked split function, splitting out of bounds is undefined behavior!
    unsafe fn split_at_unchecked(&self, mid: usize) -> (&[T], &[T])
}
```

<details>
- Sometimes we need to define a pair of functions that have very similar behavior, but one is safe, and the other one is unsafe.

- Please take the Unsafe Rust deep dive if you want to learn more about unsafe
  code. Briefly, unsafe functions transfer the responsibility for memory safety
  from the compiler to the programmer. If misused, they can trigger undefined
  behavior.

- Rust does not overload functions on safety, so we use different names for the
  functions in the pair. To make the names predictable for users, we use a
  naming convention.

- The safe function gets the short name. We add "unchecked" to the name of the
  unsafe function.

- We don't add "unchecked" to the name of every unsafe function.

  - In Rust we don't need a naming convention to highlight the danger of unsafe
    code at the callsite: Rust already requires the caller to write an
    `unsafe {}` block. This is different from other languages that don't have
    unsafe blocks, for example, Swift naming convention is to add the word
    "unsafe" to the type and function names.

  - We only use this naming convention when we want to provide a function pair,
    and therefore must use different names.

</details>
