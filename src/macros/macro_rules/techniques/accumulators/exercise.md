---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Turtle Graphics Vector

In the previous exercise, you built a recursively nested `TurtleProgram` using a
TT muncher.

In this exercise, you will adapt the `turtle_directions!` macro to use a
**Push-Down Accumulator**. Instead of nesting enum variants, your macro will
accumulate parsed `TurtleOp` variants in a bracketed buffer `[ $($acc:expr),* ]`
as it munches the input stream, ultimately constructing a `Vec<TurtleOp>`.

### Instruction DSL

The `turtle_directions!` macro parses the following unpunctuated commands into a
`Vec<TurtleOp>`:

- `FWD n` -> `TurtleOp::Forward(n)`
- `REV n` -> `TurtleOp::Reverse(n)`
- `LEFT n` -> `TurtleOp::Left(n)`
- `RIGHT n` -> `TurtleOp::Right(n)`
- `PEN UP` -> `TurtleOp::PenUp`
- `PEN DOWN` -> `TurtleOp::PenDown`
- `RED` -> `TurtleOp::SetColor(Color::Red)`
- `BLACK` -> `TurtleOp::SetColor(Color::Black)`

### Instructions:

- Implement `turtle_directions!` using internal rule push-down accumulation.
- Evaluate to `vec![ ... ]` when all tokens have been consumed.

```rust,compile_fail,editable
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum TurtleOp {
    Forward(i32),
    Reverse(i32),
    Left(i32),
    Right(i32),
    PenUp,
    PenDown,
    SetColor(Color),
}

// TODO: Implement the `turtle_directions!` macro using a push-down accumulator.

fn main() {
    let ops = turtle_directions!(
        FWD 10
        PEN UP
        REV 5
        PEN DOWN
        LEFT 90
        RED
        RIGHT 45
        BLACK
    );

    assert_eq!(
        ops,
        vec![
            TurtleOp::Forward(10),
            TurtleOp::PenUp,
            TurtleOp::Reverse(5),
            TurtleOp::PenDown,
            TurtleOp::Left(90),
            TurtleOp::SetColor(Color::Red),
            TurtleOp::Right(45),
            TurtleOp::SetColor(Color::Black),
        ]
    );
}
```

<details>

- Explain why push-down accumulation is necessary when building flat collections
  like a `Vec<T>` or array: individual macro recursive calls cannot emit looose
  comma-separated list fragments into surrounding expressions without storing
  them inside a container/buffer parameter like `[$($acc:expr),*]`.

</details>
