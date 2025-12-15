# Rust

```rust,editable
// class SelfReferentialBuffer {
//     char data[1024];
//     char* cursor;
//  
//     ...
//
// };

// Close to the original, but requires unsafe
struct SelfReferentialBuffer {
    data: [u8; 1024],
    cursor: *const u8,
}


// More idiomatic, with different semantics
struct SelfReferentialBufferSafe {
    data: [i8; 1024],
    position: usize,
}
```

<details>

While Rust would allow us to create a similar struct to the C++ class, it has a
significant cost.

We would give up references, falling back to raw pointers. This imposes unsafe
code later on.

A more idiomatic version would be to maintain an offset using a `usize`, then
creating a reference to `self` on demand.

</details>
