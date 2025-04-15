---
minutes: 5
---

# Select

A select operation waits until any of a set of futures is ready, and responds to
that future's result. In JavaScript, this is similar to `Promise.race`. In
Python, it compares to
`asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)`.

Similar to a match statement, the body of `select!` has a number of arms, each
of the form `pattern = future => statement`. When a `future` is ready, its
return value is destructured by the `pattern`. The `statement` is then run with
the resulting variables. The `statement` result becomes the result of the
`select!` macro.

```rust,editable,compile_fail
use tokio::sync::mpsc;
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);
    let listener = tokio::spawn(async move {
        tokio::select! {
            Some(msg) = rx.recv() => println!("got: {msg}"),
            _ = sleep(Duration::from_millis(50)) => println!("timeout"),
        };
    });
    sleep(Duration::from_millis(10)).await;
    tx.send(String::from("Hello!")).await.expect("Failed to send greeting");

    listener.await.expect("Listener failed");
}
```

<details>

- The `listener` async block here is a common form: wait for some async event,
  or for a timeout. Change the `sleep` to sleep longer to see it fail. Why does
  the `send` also fail in this situation?

- `select!` is also often used in a loop in "actor" architectures, where a task
  reacts to events in a loop. That has some pitfalls, which will be discussed in
  the next segment.

</details>
