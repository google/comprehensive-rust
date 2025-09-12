---
minutes: 2
---

# Safety comments

When defining unsafe functions, provide a `Safety` section in the docstring:

```rust,editable
/// Compress `data`, overwriting its memory and updating the length of the slice.
unsafe fn compress_inplace(data: &mut [u8]) {
    todo!();
}
```

When using an unsafe block, document how you have upheld your side of the
contract:

```rust,editable
unsafe {
    std::mem::transmute::<i32>(x)
}
```

<details>

## Code

```rust
/// Compress `data`, overwriting its memory and updating the length of the slice.
///
/// ## Safety
///
/// Callers must ensure that the data's compressed form is shorter than the
/// original. As a heuristic, this function should not be used on a buffer
/// that has fewer than 256 bytes.
unsafe fn compress_inplace(data: &mut [u8]) {
    todo!();
}
```

```rust
/// SAFETY: We control the generation of `x` and can ensure that it's 4 bytes wide 
unsafe {
    std::mem::transmute::<i32>(x)
}
```

> _Aside: In-place compression_
>
> Creating an algorithm that does in-place compression is likely to nerd snipe 1
> or two people. Avoid getting distracted.
>
> You could mention that it's possible to use a stack-allocated tmp buffer
> rather than something on the heap. If the implementation uses a static buffer,
> the comment must be updated to mention that the code is not thread-safe.

## Discussion

An effective safety comment is falsifiable. That is, there should be something
empirical that people can point to and check.

Note that Clippy's lint for safety comments does little more than check that the
string SAFETY: appears before the `unsafe` keyword. There is no further
validation.

</details>
