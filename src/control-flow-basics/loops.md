---
minutes: 5
---

# Loops

There are three looping keywords in Rust: `while`, `loop`, and `for`:

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

## `for`

The [`for` loop](https://doc.rust-lang.org/std/keyword.for.html) iterates over
ranges of values:

```rust,editable
fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }
}
```

## `loop`

The [`loop` statement](https://doc.rust-lang.org/std/keyword.loop.html) just
loops forever, until a `break`.

```rust,editable
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}
```

<details>

- We will discuss iteration later; for now, just stick to range expressions.
- Note that the `for` loop only iterates to `4`. Show the `1..=5` syntax for an
  inclusive range.

</details>
