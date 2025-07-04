# Unsafe Rust Functions

You can mark your own functions as `unsafe` if they require particular
preconditions to avoid undefined behaviour.

```rust,editable
/// Swaps the values pointed to by the given pointers.
///
/// # Safety
///
/// The pointers must be valid, properly aligned, and not otherwise accessed for
/// the duration of the function call.
unsafe fn swap(a: *mut u8, b: *mut u8) {
    // SAFETY: Our caller promised that the pointers are valid, properly aligned
    // and have no other access.
    unsafe {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

fn main() {
    let mut a = 42;
    let mut b = 66;

    // SAFETY: The pointers must be valid, aligned and unique because they came
    // from references.
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {}, b = {}", a, b);
}
```

<details>

We wouldn't actually use pointers for a `swap` function --- it can be done
safely with references.

Note that Rust 2021 and earlier allow unsafe code within an unsafe function
without an `unsafe` block. This changed in the 2024 edition. We can prohibit it
in older editions with `#[deny(unsafe_op_in_unsafe_fn)]`. Try adding it and see
what happens.

</details>
