# Unsafe External Functions

You can declare foreign functions for access from Rust with `unsafe extern`.
This is unsafe because the compiler has to way to reason about their behavior.
Functions declared in an `extern` block must be marked as `safe` or `unsafe`,
depending on whether they have preconditions for safe use:

```rust,editable
use std::ffi::c_char;

unsafe extern "C" {
    // `abs` doesn't deal with pointers and doesn't have any safety requirements.
    safe fn abs(input: i32) -> i32;

    /// # Safety
    ///
    /// `s` must be a pointer to a NUL-terminated C string which is valid and
    /// not modified for the duration of this function call.
    unsafe fn strlen(s: *const c_char) -> usize;
}

fn main() {
    println!("Absolute value of -3 according to C: {}", abs(-3));

    unsafe {
        // SAFETY: We pass a pointer to a C string literal which is valid for
        // the duration of the program.
        println!("String length: {}", strlen(c"String".as_ptr()));
    }
}
```

<details>

- Rust used to consider all extern functions unsafe, but this changed in Rust
  1.82 with `unsafe extern` blocks.
- `abs` must be explicitly marked as `safe` because it is an external function
  (FFI). Calling external functions is usually only a problem when those
  functions do things with pointers which might violate Rust's memory model, but
  in general any C function might have undefined behaviour under any arbitrary
  circumstances.
- The `"C"` in this example is the ABI;
  [other ABIs are available too](https://doc.rust-lang.org/reference/items/external-blocks.html).
- Note that there is no verification that the Rust function signature matches
  that of the function definition -- that's up to you!

</details>
