# Documentation Tests

Rust has built-in support for documentation tests:

```rust
/// Shortens a string to the given length.
///
/// ```
/// # use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
```

* Code blocks in `///` comments are automatically seen as Rust code.
* The code will be compiled and executed as part of `cargo test`.
* Adding `# ` in the code will hide it from the docs, but will still compile/run it.
* Test the above code on the [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=3ce2ad13ea1302f6572cb15cd96becf0).
