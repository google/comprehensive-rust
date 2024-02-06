# `for`

The [`for` loop](https://doc.rust-lang.org/std/keyword.for.html) iterates over
ranges of values:

```rust,editable
fn main() {
    for x in 1..5 {
        println!("x: {x}");
    }
}
```

<details>

- We will discuss iteration later; for now, just stick to range expressions.
- Note that the `for` loop only iterates to `4`. Show the `1..=5` syntax for an
  inclusive range.

</details>
