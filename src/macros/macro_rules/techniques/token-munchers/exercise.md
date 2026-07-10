---
minutes: 10
---

<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Exercise: Turtle Graphics

In this exercise, you will create an incremental TT muncher macro called
`turtle_directions!`.

Your macro will parse a domain-specific language (DSL) for turtle graphics
instructions which maintains its old-school style by omitting any punctuation
between instructions. This DSL will be used to construct a `cons`-list-style
`TurtleProgram` value.

### Turtle Operation DSL

The `turtle_directions!` macro should parse the following unpunctuated commands:

- `FWD n`: moves forward by `n` steps (`TurtleProgram::Forward(n, rest)`)
- `REV n`: moves backward by `n` steps (`TurtleProgram::Reverse(n, rest)`)
- `LEFT n`: turns left by `n` degrees (`TurtleProgram::Left(n, rest)`)
- `RIGHT n`: turns right by `n` degrees (`TurtleProgram::Right(n, rest)`)
- `PEN UP`: lifts the pen (`TurtleProgram::PenUp(rest)`)
- `PEN DOWN`: lowers the pen (`TurtleProgram::PenDown(rest)`)
- `RED` and `BLACK`: set the pen color (`TurtleProgram::SetColor(color, rest)`)

Where `rest` is a `Box<TurtleProgram>` representing the remaining steps of the
program.

Implement the `turtle_directions!` macro in the following program:

```rust,compile_fail,editable
#[derive(Debug, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum TurtleProgram {
    Forward(i32, Box<TurtleProgram>),
    Reverse(i32, Box<TurtleProgram>),
    Left(i32, Box<TurtleProgram>),
    Right(i32, Box<TurtleProgram>),
    PenUp(Box<TurtleProgram>),
    PenDown(Box<TurtleProgram>),
    SetColor(Color, Box<TurtleProgram>),
    Done,
}

// TODO: Implement the `turtle_directions!` macro.

fn main() {
    let program = turtle_directions!(
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
        program,
        TurtleProgram::Forward(
            10,
            Box::new(TurtleProgram::PenUp(Box::new(TurtleProgram::Reverse(
                5,
                Box::new(TurtleProgram::PenDown(Box::new(TurtleProgram::Left(
                    90,
                    Box::new(TurtleProgram::SetColor(
                        Color::Red,
                        Box::new(TurtleProgram::Right(
                            45,
                            Box::new(TurtleProgram::SetColor(
                                Color::Black,
                                Box::new(TurtleProgram::Done)
                            ))
                        ))
                    ))
                ))))
            ))))
        )
    );
}
```

<details>

- Highlight how matching on specific keyword token sequences (e.g. `PEN UP`,
  `RED`, or `FWD n`) at the head of the token stream allows parsing each command
  according to the number and kind of expected tokens to follow.

</details>
