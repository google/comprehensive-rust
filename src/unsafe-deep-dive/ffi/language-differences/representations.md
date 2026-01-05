# Different representations

```rust,editable
fn main() {
    let c_repr = b"Hello, C\0";
    let cc_repr = (b"Hello, C++\0", 10u32);
    let rust_repr = (b"Hello, Rust", 11);
}
```

<details>

Each language has its own opinion about how to implement things, which can lead
to confusion and bugs. Consider three ways to represent text.

Show how to convert the raw representations to a Rust string slice:

```rust
// C representation to Rust
unsafe {
    let ptr = c_repr.as_ptr() as *const i8;
    let c: &str = std::ffi::CStr::from_ptr(ptr).to_str().unwrap();
    println!("{c}");
};

// C++ representation to Rust 
unsafe {
    let ptr = cc_repr.0.as_ptr();
    let bytes = std::slice::from_raw_parts(ptr, cc_repr.1);
    let cc: &str = std::str::from_utf8_unchecked(bytes);
    println!("{cc}");
};

// Rust representation (bytes) to string slice
unsafe {
    let ptr = rust_repr.0.as_ptr();
    let bytes = std::slice::from_raw_parts(ptr, rust_repr.1);
    let rust: &str = std::str::from_utf8_unchecked(bytes);
    println!("{rust}");
};
```

Aside: Rust has a c-prefixed string literal. It appends a null byte at the end,
e.g. `c"Rust" == b"Rust\0"`.

</details>
