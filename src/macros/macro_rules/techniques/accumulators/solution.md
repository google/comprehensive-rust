<!--
Copyright 2026 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Solution

Here is the implementation of the `turtle_directions!` macro using a push-down
accumulator:

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
pub enum TurtleOp {
    Forward(i32),
    Reverse(i32),
    Left(i32),
    Right(i32),
    PenUp,
    PenDown,
    SetColor(Color),
}

macro_rules! turtle_directions {
    // Base case: no more tokens, expand accumulated operations into a Vec
    (@accum [ $($acc:expr),* $(,)? ]) => {
        vec![ $($acc),* ]
    };

    // Munch FWD n
    (@accum [ $($acc:expr),* ] FWD $n:tt $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::Forward($n) ] $($rest)*)
    };

    // Munch REV n
    (@accum [ $($acc:expr),* ] REV $n:tt $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::Reverse($n) ] $($rest)*)
    };

    // Munch LEFT n
    (@accum [ $($acc:expr),* ] LEFT $n:tt $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::Left($n) ] $($rest)*)
    };

    // Munch RIGHT n
    (@accum [ $($acc:expr),* ] RIGHT $n:tt $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::Right($n) ] $($rest)*)
    };

    // Munch PEN UP
    (@accum [ $($acc:expr),* ] PEN UP $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::PenUp ] $($rest)*)
    };

    // Munch PEN DOWN
    (@accum [ $($acc:expr),* ] PEN DOWN $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::PenDown ] $($rest)*)
    };

    // Munch RED
    (@accum [ $($acc:expr),* ] RED $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::SetColor(Color::Red) ] $($rest)*)
    };

    // Munch BLACK
    (@accum [ $($acc:expr),* ] BLACK $($rest:tt)*) => {
        turtle_directions!(@accum [ $($acc,)* TurtleOp::SetColor(Color::Black) ] $($rest)*)
    };

    // Entry point: initialize push-down accumulator
    ($($tokens:tt)*) => {
        turtle_directions!(@accum [] $($tokens)*)
    };
}

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

The push-down accumulator works as follows:

1. The public entry point rule matching `$($tokens:tt)*` initializes the
   push-down buffer by delegating to `@accum [] $($tokens)*`.
2. Each internal rule matches a command at the head of the token stream and
   appends the corresponding `TurtleOp` variant to the buffer.
3. The remaining tokens `$($rest:tt)*` are recursively passed back into
   `turtle_directions!(@accum ...)`.
4. When all tokens are munched, the base case `@accum [ $($acc:expr),* ]`
   expands the accumulated buffer into a `vec![ $($acc),* ]`.

</details>
