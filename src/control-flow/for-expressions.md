# `for` expressions

The `for` expression is closely related to the `while let` expression. It will
automatically call `into_iter()` on the expression and then iterate over it:

```rust,editable
fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }
}
```

You can use `break` and `continue` here as usual.
