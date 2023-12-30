# `Pin`

When you await a future, all local variables (that would ordinarily be stored on
a stack frame) are instead stored in the Future for the current async block. If
your future has pointers to data on the stack, those pointers might get
invalidated. This is unsafe.

Therefore, you must guarantee that the addresses your future points to don't
change. That is why we need to "pin" futures. Using the same future repeatedly
in a `select!` often leads to issues with pinned values.

```rust,editable,compile_fail
use tokio::sync::{mpsc, oneshot};
use tokio::task::spawn;
use tokio::time::{sleep, Duration};

// A work item. In this case, just sleep for the given time and respond
// with a message on the `respond_on` channel.
#[derive(Debug)]
struct Work {
    input: u32,
    respond_on: oneshot::Sender<u32>,
}

// A worker which listens for work on a queue and performs it.
async fn worker(mut work_queue: mpsc::Receiver<Work>) {
    let mut iterations = 0;
    loop {
        tokio::select! {
            Some(work) = work_queue.recv() => {
                sleep(Duration::from_millis(10)).await; // Pretend to work.
                work.respond_on
                    .send(work.input * 1000)
                    .expect("failed to send response");
                iterations += 1;
            }
            // TODO: report number of iterations every 100ms
        }
    }
}

// A requester which requests work and waits for it to complete.
async fn do_work(work_queue: &mpsc::Sender<Work>, input: u32) -> u32 {
    let (tx, rx) = oneshot::channel();
    work_queue
        .send(Work { input, respond_on: tx })
        .await
        .expect("failed to send on work queue");
    rx.await.expect("failed waiting for response")
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(10);
    spawn(worker(rx));
    for i in 0..100 {
        let resp = do_work(&tx, i).await;
        println!("work result for iteration {i}: {resp}");
    }
}
```

<details>

- You may recognize this as an example of the actor pattern. Actors typically
  call `select!` in a loop.

- This serves as a summation of a few of the previous lessons, so take your time
  with it.

  - Naively add a `_ = sleep(Duration::from_millis(100)) => { println!(..) }` to
    the `select!`. This will never execute. Why?

  - Instead, add a `timeout_fut` containing that future outside of the `loop`:

    ```rust,compile_fail
    let mut timeout_fut = sleep(Duration::from_millis(100));
    loop {
        select! {
            ..,
            _ = timeout_fut => { println!(..); },
        }
    }
    ```
  - This still doesn't work. Follow the compiler errors, adding `&mut` to the
    `timeout_fut` in the `select!` to work around the move, then using
    `Box::pin`:

    ```rust,compile_fail
    let mut timeout_fut = Box::pin(sleep(Duration::from_millis(100)));
    loop {
        select! {
            ..,
            _ = &mut timeout_fut => { println!(..); },
        }
    }
    ```

  - This compiles, but once the timeout expires it is `Poll::Ready` on every
    iteration (a fused future would help with this). Update to reset
    `timeout_fut` every time it expires.

- Box allocates on the heap. In some cases, `std::pin::pin!` (only recently
  stabilized, with older code often using `tokio::pin!`) is also an option, but
  that is difficult to use for a future that is reassigned.

- Another alternative is to not use `pin` at all but spawn another task that
  will send to a `oneshot` channel every 100ms.

</details>
