---
minutes: 5
existing course material:
- control-flow/while-expressions.md
- control-flow/for-expressions.md
- control-flow/loop-expressions.md
---

# Loops

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

# `for` loops

The [`for` loop](https://doc.rust-lang.org/std/keyword.for.html) is closely
related to the [`while let` loop](while-let-expressions.md). It will
automatically call `into_iter()` on the expression and then iterate over it:

<!-- mdbook-xgettext: skip -->
```rust,editable
fn main() {
    let v = vec![10, 20, 30];

    for x in v {
        println!("x: {x}");
    }

    for i in (0..10).step_by(2) {
        println!("i: {i}");
    }
}
```

You can use `break` and `continue` here as usual.

<details>

* Index iteration is not a special syntax in Rust for just that case.
* `(0..10)` is a range that implements an `Iterator` trait.
* `step_by` is a method that returns another `Iterator` that skips every other element.
* Modify the elements in the vector and explain the compiler errors. Change vector `v` to be mutable and the for loop to `for x in v.iter_mut()`.

</details>
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
