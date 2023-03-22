# `async`/`await`

At a high level, async Rust code looks very much like "normal" sequential code:

```rust,editable,compile_fail
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("Count in task: {i}!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(10));

    for i in 1..5 {
        println!("Main task: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}
```

<details>

Key points:

* Tokio is one of several async runtimes available for Rust.

* The function is decorated with the "async" keyword to indicate that it is async.  The
  `tokio::main` macro invocation is a convenience to wrap the `main` function as a task.

* The `spawn` function creates a new, concurrent "task", just like spawning a thread.

* Whenever a task would block, we add an `.await` which returns control to the runtime until the
  blocking operation is ready to proceed.

Further exploration:

* Why does `count_to` not (usually) get to 10? This is an example of async cancellation.
  `tokio::spawn` returns a handle which can be awaited to wait until it finishes.

* Try `count_to(10).await` instead of spawning.

* Try importing `tokio::join` and using it to join multiple handles.

Note that the Rust playground does not allow network connections, so examples like making HTTP
requests are not possible.

</details>
