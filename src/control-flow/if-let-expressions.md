# `if let` expressions

If you want to match a value against a pattern, you can use `if let`:

```rust,editable
fn main() {
    let arg = std::env::args().next();
    if let Some(value) = arg {
        println!("Program name: {value}");
    } else {
        println!("Missing name?");
    }
}
```

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.
