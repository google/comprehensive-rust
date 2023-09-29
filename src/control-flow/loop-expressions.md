# `loop` expressions

Finally, there is a [`loop` keyword](https://doc.rust-lang.org/reference/expressions/loop-expr.html#infinite-loops)
which creates an endless loop.

Here you must either `break` or `return` to stop the loop:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 {
            x / 2
        } else {
            3 * x + 1
        };
        if x == 1 {
            break;
        }
    }
    println!("x: {x}");
}
```

<details>
    
* Break the `loop` with a value (e.g. `break 8`) and print it out.
* Note that `loop` is the only looping construct which returns a non-trivial
  value. This is because it's guaranteed to be entered at least once (unlike
  `while` and `for` loops).

</details>
