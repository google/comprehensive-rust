# `break` and `continue`

- If you want to exit a loop early, use [`break`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions),
- If you want to immediately start
the next iteration use [`continue`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions).

Both `continue` and `break` can optionally take a label argument which is used
to break out of nested loops:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}
```

In this case we break the outer loop after 3 iterations of the inner loop.
