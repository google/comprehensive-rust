# `match` expressions

The `match` keyword is used to match a value against one or more patterns. In
that sense, it works like a series of `if let` expressions:

```rust,editable
fn main() {
    let args: Vec<String> = std::env::args().collect();

    for s in &args[1..] {
        match &s[..] {
            "cat" => println!("Will do cat things"),
            "ls" => println!("Will ls some files"),
            "mv" => println!("Let's move some files"),
            "rm" => println!("Uh, dangerous!"),
            _ => println!("Unknown program name!"),
        }
    }
}
```

Like `if let`, each match arm must have the same type. The type is the last
expression of the block, if any. In the example above, the type is `()`.

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.
