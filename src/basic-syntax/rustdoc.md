# Rustdoc

All language items in Rust can be documented using special `///` syntax.

```rust,editable
/// Determine whether the first argument is divisible by the second argument.
///
/// If the second argument is zero, the result is false.
///
/// # Example
/// ```
/// assert!(is_divisible_by(42, 2));
/// ```
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;  // Corner case, early return
    }
    lhs % rhs == 0     // The last expression in a block is the return value
}
```

The contents are treated as Markdown. All published Rust library crates are
automatically documented at [`docs.rs`](https://docs.rs) using the
[rustdoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) tool. It is
idiomatic to document all public items in an API using this pattern.
Code snippets can document usage and will be used as unit tests.

<details>

* Show students the generated docs for the `rand` crate at
  [`docs.rs/rand`](https://docs.rs/rand).

* This course does not include rustdoc on slides, just to save space, but in
  real code they should be present.

* Inner doc comments are discussed later (in the page on modules) and need not
  be addressed here.

* Rustdoc comments can contain code snippets that we can run and test using `cargo test`.
  We will discuss these tests in the [Testing section](../testing/doc-tests.html).

</details>
