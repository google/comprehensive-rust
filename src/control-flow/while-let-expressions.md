# `while let` loops

Like with `if let`, there is a [`while let`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-pattern-loops)
variant which repeatedly tests a value against a pattern:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();

    while let Some(x) = iter.next() {
        println!("x: {x}");
    }
}
```

Here the iterator returned by `v.into_iter()` will return a `Option<i32>` on every
call to `next()`. It returns `Some(x)` until it is done, after which it will
return `None`. The `while let` lets us keep iterating through all items.

See [pattern matching](../pattern-matching.md) for more details on patterns in
Rust.

<details>

* Point out that the `while let` loop will keep going as long as the value matches the pattern.
* You could rewrite the `while let` loop as an infinite loop with an if statement that breaks when there is no value to unwrap for `iter.next()`. The `while let` provides syntactic sugar for the above scenario.
    
</details>
