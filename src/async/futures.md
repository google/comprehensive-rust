# Futures

What is the type of an async operation?

```rust,editable,compile_fail
use tokio::time;

async fn count_to(count: i32) -> i32 {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
    count
}

#[tokio::main]
async fn main() {
    println!("Final count is: {}!", count_to(13).await);

    // Uncomment the following line to see the return type of the async call.
    // let _: () = count_to(13);

}
```

[Future](https://doc.rust-lang.org/nightly/src/core/future/future.rs.html#37)
is a trait, implemented by objects that represent an operation that may not be
complete yet. A future can be polled, and `poll` returns either
`Poll::Ready(result)` or `Poll::Pending`.

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

An async function returns an `impl Future`, and an async block evaluates to an
`impl Future`. It's also possible (but uncommon) to implement `Future` for your
own types. For example, the `JoinHandle` returned from `tokio::spawn` implements
`Future` to allow joining to it.

The `.await` keyword, applied to a Future, causes the current async function or
block to pause until that Future is ready, and then evaluates to its output.

<details>

* The `Future` and `Poll` types are conceptually quite simple, and implemented as
  such in `std::task`.

* We will not get to `Pin` and `Context`, as we will focus on writing async
  code, rather than building new async primitives. Briefly:

  * `Context` allows a Future to schedule itself to be polled again when an
    event occurs.

  * `Pin` ensures that the Future isn't moved in memory, so that pointers into
    that future remain valid. This is required to allow references to remain
    valid after an `.await`.

</details>
