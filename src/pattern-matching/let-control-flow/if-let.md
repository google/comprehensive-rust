# `if let` Expressions

The
[`if let` expression](https://doc.rust-lang.org/reference/expressions/if-expr.html#if-let-expressions)
lets you execute different code depending on whether a value matches a pattern:

```rust,editable
use std::time::Duration;

fn sleep_for(secs: f32) {
    if let Ok(duration) = Duration::try_from_secs_f32(secs) {
        std::thread::sleep(duration);
        println!("slept for {duration:?}");
    }
}

fn main() {
    sleep_for(-10.0);
    sleep_for(0.8);
}
```

<details>

- Unlike `match`, `if let` does not have to cover all branches. This can make it
  more concise than `match`.
- A common usage is handling `Some` values when working with `Option`.
- Unlike `match`, `if let` does not support guard clauses for pattern matching.
- With an `else` clause, this can be used as an expression.

</details>
