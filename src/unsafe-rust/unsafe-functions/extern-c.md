# Unsafe External Functions

All functions implemented in a foreign language are considered unsafe in Rust:

```rust,editable
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    // SAFETY: `abs` doesn't deal with pointers and doesn't have any safety
    // requirements.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

<details>

`abs` is unsafe because it is an external function (FFI). Calling external
functions is usually only a problem when those functions do things with pointers
which might violate Rust's memory model, but in general any C function might
have undefined behaviour under any arbitrary circumstances.

The `"C"` in this example is the ABI;
[other ABIs are available too](https://doc.rust-lang.org/reference/items/external-blocks.html).

Note that there is no verification that the Rust function signature matches that
of the function definition -- that's up to you!

</details>
