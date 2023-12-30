# Futures

[`Future`](https://doc.rust-lang.org/std/future/trait.Future.html) is a trait,
implemented by objects that represent an operation that may not be complete yet.
A future can be polled, and `poll` returns a
[`Poll`](https://doc.rust-lang.org/std/task/enum.Poll.html).

```rust
use std::pin::Pin;
use std::task::Context;

pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

An async function returns an `impl Future`. It's also possible (but uncommon) to
implement `Future` for your own types. For example, the `JoinHandle` returned
from `tokio::spawn` implements `Future` to allow joining to it.

The `.await` keyword, applied to a Future, causes the current async function to
pause until that Future is ready, and then evaluates to its output.

<details>

- The `Future` and `Poll` types are implemented exactly as shown; click the
  links to show the implementations in the docs.

- We will not get to `Pin` and `Context`, as we will focus on writing async
  code, rather than building new async primitives. Briefly:

  - `Context` allows a Future to schedule itself to be polled again when an
    event occurs.

  - `Pin` ensures that the Future isn't moved in memory, so that pointers into
    that future remain valid. This is required to allow references to remain
    valid after an `.await`.

</details>
