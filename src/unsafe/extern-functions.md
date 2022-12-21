# Calling External Code

Functions from other languages might violate the guarantees of Rust. Calling
them is thus unsafe:

```rust,editable
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        // Undefined behavior if abs misbehaves.
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```
