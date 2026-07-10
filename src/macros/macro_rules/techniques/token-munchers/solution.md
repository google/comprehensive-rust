<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

Here is the implementation of the `turtle_directions!` TT muncher macro:

```rust,editable
# // Copyright 2026 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
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

use Color::*;

macro_rules! turtle_directions {
    () => {
        TurtleProgram::Done
    };
    (FWD $n:tt $($rest:tt)*) => {
        TurtleProgram::Forward($n, Box::new(turtle_directions!($($rest)*)))
    };
    (REV $n:tt $($rest:tt)*) => {
        TurtleProgram::Reverse($n, Box::new(turtle_directions!($($rest)*)))
    };
    (LEFT $n:tt $($rest:tt)*) => {
        TurtleProgram::Left($n, Box::new(turtle_directions!($($rest)*)))
    };
    (RIGHT $n:tt $($rest:tt)*) => {
        TurtleProgram::Right($n, Box::new(turtle_directions!($($rest)*)))
    };
    (PEN UP $($rest:tt)*) => {
        TurtleProgram::PenUp(Box::new(turtle_directions!($($rest)*)))
    };
    (PEN DOWN $($rest:tt)*) => {
        TurtleProgram::PenDown(Box::new(turtle_directions!($($rest)*)))
    };
    (RED $($rest:tt)*) => {
        TurtleProgram::SetColor(Red, Box::new(turtle_directions!($($rest)*)))
    };
    (BLACK $($rest:tt)*) => {
        TurtleProgram::SetColor(Black, Box::new(turtle_directions!($($rest)*)))
    };
}

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
                        Red,
                        Box::new(TurtleProgram::Right(
                            45,
                            Box::new(TurtleProgram::SetColor(
                                Black,
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

The macro processes the input sequence incrementally:

1. Each rule matches a specific command pattern at the head of the token stream
   (such as `FWD $n:tt`, `PEN UP`, or `PEN DOWN`).
2. The matched command is wrapped into its corresponding `TurtleProgram`
   variant, passing `Box::new(turtle_directions!($($rest)*))` as the rest of the
   program.
3. Recursion continues on the remaining tokens `$($rest:tt)*` until no tokens
   remain, matching the empty `()` base case and giving `TurtleProgram::Done`.

Note that `$n:tt` is used for numeric arguments because the built-in follow-set
rules prohibit `$($rest:tt)*` from directly following `$n:expr` without
punctuation.

## Extra Credit

Consider how the definition of this macro could be further factored to avoid the
repetition of `TurtleProgram::$OPERATION($ARG, Box::new(turtle_directions!(` in
most of its arms.
