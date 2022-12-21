# Documentation Tests

Rust has built-in support for documentation tests:

```rust
/// Shorten string will trip the string to the given length.
///
/// ```
/// use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
```

* Code blocks in `///` comments are automatically seen as Rust code.
* The code will be compiled and executed as part of `cargo test`.
* Test the above code on the [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c8ce535a3778218fed50c2b4c317d15d).
