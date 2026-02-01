---
minutes: 4
---

# Structs

Like tuples, structs can also be destructured by matching:

```rust,editable
struct Move {
    delta: (i32, i32),
    repeat: u32,
}

#[rustfmt::skip]
fn main() {
    let m = Move { delta: (10, 0), repeat: 5 };

    match m {
        Move { delta: (0, 0), .. }        => println!("Standing still"),
        Move { delta: (x, 0), repeat }    => println!("{repeat} step x: {x}"),
        Move { delta: (0, y), repeat: 1 } => println!("Single step y: {y}"),
        _                                 => println!("Other move"),
    }
}
```

<details>

- Change the literal values in `m` to match with the other patterns.
- Add a new field to `Movement` and make changes to the pattern as needed.
- Note how `delta: (x, 0)` is a nested pattern.

## More to Explore

- Try `match &m` and check the type of captures. The pattern syntax remains the
  same, but the captures become shared references. This is
  [match ergonomics](https://rust-lang.github.io/rfcs/2005-match-ergonomics.html)
  and is often useful with `match self` when implementing methods on an enum.
  - The same effect occurs with `match &mut m`: the captures become exclusive
    references.
- The distinction between a capture and a constant expression can be hard to
  spot. Try changing the `10` in the first arm to a variable, and see that it
  subtly doesn't work. Change it to a `const` and see it working again.

</details>
