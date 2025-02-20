---
minutes: 3
---

# Closure Syntax

Closures are created with vertical bars: `|..| ..`.

```rust,editable
fn main() {
    // Argument and return type can be inferred for lightweight syntax:
    let double_it = |n| n * 2;
    dbg!(double_it(50));

    // Or we can specify types and bracket the body to be fully explicit:
    let add_1f32 = |x: f32| -> f32 { x + 1.0 };
    dbg!(add_1f32(50.));
}
```

<details>

- The arguments go between the `|..|`. The body can be surrounded by `{ .. }`,
  but if it is a single expression these can be omitted.

- Argument types are optional, and are inferred if not given. The return type is
  also optional, but can only be written if using `{ .. }` around the body.

- The examples can both be written as mere nested functions instead -- they do
  not capture any variables from their lexical environment. We will see captures
  next.

</details>
