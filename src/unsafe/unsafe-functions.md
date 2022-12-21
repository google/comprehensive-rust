# Calling Unsafe Functions

A function or method can be marked `unsafe` if it has extra preconditions you
must uphold:

```rust,editable
fn main() {
    let emojis = "🗻∈🌏";
    unsafe {
        // Undefined behavior if indices do not lie on UTF-8 sequence boundaries.
        println!("{}", emojis.get_unchecked(0..4));
        println!("{}", emojis.get_unchecked(4..7));
        println!("{}", emojis.get_unchecked(7..11));
    }
}
```
