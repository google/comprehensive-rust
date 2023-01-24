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

<details>

* `if let` can be more concise than `match`, e.g., when only one case is interesting. In contrast, `match` requires all branches to be covered.
    * For the similar use case consider demonstrating a newly stabilized [`let else`](https://github.com/rust-lang/rust/pull/93628) feature.
* A common usage is handling `Some` values when working with `Option`.
* Unlike `match`, `if let` does not support guard clauses for pattern matching.

</details>
