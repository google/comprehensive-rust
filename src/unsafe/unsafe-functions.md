# Calling Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold to avoid undefined behaviour:

```rust,editable
fn main() {
    let emojis = "🗻∈🌏";

    // Safe because the indices are in the correct order, within the bounds of
    // the string slice, and lie on UTF-8 sequence boundaries.
    unsafe {
        println!("{}", emojis.get_unchecked(0..4));
        println!("{}", emojis.get_unchecked(4..7));
        println!("{}", emojis.get_unchecked(7..11));
    }
}
```
