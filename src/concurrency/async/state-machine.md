---
minutes: 10
---

# State Machine

Rust transforms an async function or block to a hidden type that implements
`Future`, using a state machine to track the function's progress. The details of
this transform are complex, but it helps to have a schematic understanding of
what is happening.

```rust,editable
use std::task::Poll;

/// A simplified version of `std::future::Future`.
trait Future {
    type Output;
    fn poll(&mut self) -> Poll<Self::Output>;
}

/*
 * async fn roll_d10() -> u32 { 7 }
 */

/// Return a random number from 1 to 10.
fn roll_d10() -> RollD10 {
    RollD10
}

struct RollD10;

impl Future for RollD10 {
    type Output = u32;

    fn poll(&mut self) -> Poll<Self::Output> {
        Poll::Ready(7)
    }
}

/*
 * async fn two_d10(modifier: u32) -> u32 {
 *     let first_roll = roll_d10().await;
 *     let second_roll = roll_d10().await;
 *     first_roll + second_roll + modifier
 * }
 */

/// Sum two D10 rolls plus a modifier.
fn two_d10(modifier: u32) -> TwoD10 {
    TwoD10::Init { modifier }
}

enum TwoD10 {
    // Function has not begun yet.
    Init { modifier: u32 },
    // Waitig for first `.await` to complete.
    FirstRoll { modifier: u32, fut: RollD10 },
    // Waitig for second `.await` to complete.
    SecondRoll { modifier: u32, first_roll: u32, fut: RollD10 },
}

impl Future for TwoD10 {
    type Output = u32;
    fn poll(&mut self) -> Poll<Self::Output> {
        loop {
            match self {
                TwoD10::Init { modifier } => {
                    let fut = roll_d10();
                    *self = TwoD10::FirstRoll { modifier: *modifier, fut };
                }
                TwoD10::FirstRoll { modifier, fut } => {
                    if let Poll::Ready(first_roll) = fut.poll() {
                        let fut = roll_d10();
                        *self = TwoD10::SecondRoll {
                            modifier: *modifier,
                            first_roll,
                            fut,
                        };
                    } else {
                        return Poll::Pending;
                    }
                }
                TwoD10::SecondRoll { modifier, first_roll, fut } => {
                    if let Poll::Ready(second_roll) = fut.poll() {
                        return Poll::Ready(*first_roll + second_roll + *modifier);
                    } else {
                        return Poll::Pending;
                    }
                }
            }
        }
    }
}

fn main() {
    let mut fut = two_d10(13);
    loop {
        if let Poll::Ready(result) = fut.poll() {
            println!("result: {result}");
            break;
        }
    }
}
```

<details>

This example is illustrative, and isn't an accurate representation of the Rust
compiler's transformation. The `Future` type in this example omits `Pin` and
`Context` for simplicity.

The important things to notice here are:

- Calling an async function does nothing but construct and return a future.
- All local variables are stored in the function's future, using an enum to
  identify where execution is currently suspended.
- An `.await` in the async function is translated into an a new state containing
  all live variables and the awaited future. The `loop` then handles that
  updated state, polling the future until it returns `Poll::Ready`.
- Execution continues eagerly until a `Poll::Pending` occurs. In this simple
  example, every future is ready immediately.
- `main` contains a naïve executor, which just busy-loops until the future is
  ready. We will discuss real executors shortly.

# More to Explore

Imagine the `Future` data structure for a deeply nested stack of async
functions. Each function's `Future` contains the `Future` structures for the
functions it calls. This can result in unexpectedly large compiler-generated
`Future` types.

This also means that recursive async functions are challenging. Compare to the
common error of building recursive type, such as

```rust,compile_fail
enum LinkedList<T> {
    Node { value: T, next: LinkedList<T> },
    Nil,
}
```

The fix for a recursive type is to add a layer of indrection, such as with
`Box`. Similarly, a recursive async function must box the recursive future:

```rust,editable
async fn count_to(n: u32) {
    if n > 0 {
        Box::pin(count_to(n - 1)).await;
        println!("{n}");
    }
}
```

</details>
