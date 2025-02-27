# `loop`

The [`loop` statement](https://doc.rust-lang.org/std/keyword.loop.html) just
loops forever, until a `break`.

```rust,editable
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        dbg!(i);
        if i > 100 {
            break;
        }
    }
}
```

<details>

- The `loop` statement works like a `while true` loop. Use it for things like
  servers which will serve connections forever.

## More to Explore

- `loop` is the only form of loop that can return a value. If students are
  curious, demonstrate breaking with a value, e.g. `break x + 10`.

</details>
