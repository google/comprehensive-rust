---
minutes: 5
---

# Capturing

A closure can capture variables from the environment where it was defined.

```rust,editable
fn main() {
    let max_value = 5;
    let clamp = |v| {
        if v > max_value { max_value } else { v }
    };

    dbg!(clamp(1));
    dbg!(clamp(3));
    dbg!(clamp(5));
    dbg!(clamp(7));
    dbg!(clamp(10));
}
```

<details>

- By default, a closure captures values by reference. Here `max_value` is
  captured by `clamp`, but still available to `main` for printing. Try making
  `max_value` mutable, changing it, and printing the clamped values again. Why
  doesn't this work?

- If a closure mutates values, it will capture them by mutable reference. Try
  adding `max_value += 1` to `clamp`.

- You can force a closure to move values instead of referencing them with the
  `move` keyword. This can help with lifetimes, for example if the closure must
  outlive the captured values (more on lifetimes later).

  This looks like `move |v| ..`. Try adding this keyword and see if `main` can
  still access `max_value` after defining `clamp`.

- By default, closures will capture each variable from an outer scope by the
  least demanding form of access they can (by shared reference if possible, then
  exclusive reference, then by move). The `move` keyword forces capture by
  value.

</details>
