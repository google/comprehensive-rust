---
minutes: 3
---

# Closure Syntax

Closures are created with vertical bars: `|..| ..`.

```rust,editable
fn main() {
    let value = Some(13);
    dbg!(value.map(|num| format!("{num}")));

    let mut nums = vec![1, 10, 99, 24];
    // Sort even numbers first.
    nums.sort_by_key(|v| if v % 2 == 0 { (0, *v) } else { (1, *v) });
    dbg!(nums);
}
```

<details>

- The arguments go between the `|..|`. The body can be surrounded by `{ .. }`,
  but if it is a single expression these can be omitted.

- Argument types are optional, and are inferred if not given. The return type is
  also optional, but can only be written if using `{ .. }` around the body.

- The examples are both lambdas -- they do not capture anything from their
  environment. We will see captures next.

</details>
