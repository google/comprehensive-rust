---
minutes: 5
---

## `while`

The
[`while` keyword](https://doc.rust-lang.org/reference/expressions/loop-expr.html#predicate-loops)
works much like in other languages, executing the loop body as long as the
condition is true.

```rust,editable
fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}
```

