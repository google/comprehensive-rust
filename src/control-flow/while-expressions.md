# `while` loops

The [`while` keyword](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops)
works very similar to other languages:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let mut x = 10;
    while x != 1 {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
    }
    println!("x: {x}");
}
```

