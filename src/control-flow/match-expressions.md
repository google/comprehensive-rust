# `match` expressions

The `match` keyword is used to match a value against one or more patterns. In
that sense, it works like a series of `if let` expressions:

```rust,editable
fn main() {
    match std::env::args().next().as_deref() {
        Some("cat") => println!("Will do cat things"),
        Some("ls")  => println!("Will ls some files"),
        Some("mv")  => println!("Let's move some files"),
        Some("rm")  => println!("Uh, dangerous!"),
        None        => println!("Hmm, no program name?"),
        _           => println!("Unknown program name!"),
    }
}
```

Like `if let`, each match arm must have the same type. The type is the last
expression of the block, if any. In the example above, the type is `()`.

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

* Save the match expression to a variable and print it out.
* Remove `.as_deref()` and explain the error.
    * `std::env::args().next()` returns an `Option<String>`, but we cannot match against `String`.
    * `as_deref()` transforms an `Option<T>` to `Option<&T::Target>`. In our case, this turns `Option<String>` into `Option<&str>`.
    * We can now use pattern matching to match against the `&str` inside `Option`.
</details>
