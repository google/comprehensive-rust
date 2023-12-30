---
minutes: 5
---

# `break` and `continue`

If you want to exit any kind of loop early, use
[`break`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#break-expressions).
For `loop`, this can take an optional expression that becomes the value of the
`loop` expression.

If you want to immediately start the next iteration use
[`continue`](https://doc.rust-lang.org/reference/expressions/loop-expr.html#continue-expressions).

```rust,editable
fn main() {
    let (mut a, mut b) = (100, 52);
    let result = loop {
        if a == b {
            break a;
        }
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };
    println!("{result}");
}
```

Both `continue` and `break` can optionally take a label argument which is used
to break out of nested loops:

```rust,editable
fn main() {
    'outer: for x in 1..5 {
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

<details>

- Note that `loop` is the only looping construct which returns a non-trivial
  value. This is because it's guaranteed to be entered at least once (unlike
  `while` and `for` loops).

</details>
