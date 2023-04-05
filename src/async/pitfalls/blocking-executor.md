# Blocking the executor

Most async runtimes only allow IO tasks to run concurrently.
This means that CPU blocking tasks will block the executor and prevent other tasks from being executed.
An easy workaround is to use async equivalent methods where possible.

```rust,editable,compile_fail
use futures::future::join_all;
use std::time::Instant;

fn sleep_ms(start: &Instant, id: u64, duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    println!(
        "future {id} slept for {duration_ms}ms, finished after {}ms",
        start.elapsed().as_millis()
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    let sleep_futures = (1..=10).map(|t| sleep_ms(&start, t, t * 10));
    join_all(sleep_futures).await;
}
```

<details>

* Run the code and see that the sleeps happen consecutively rather than
  concurrently.

* The `"current_thread"` flavor puts all tasks on a single thread. This makes the
  effect more obvious, but the bug is still present in the multi-threaded
  flavor.

* Switch the `std::thread::sleep` to `tokio::time::sleep` and await its result.

* Another fix would be to `tokio::task::spawn_blocking` which spawns an actual
  thread and transforms its handle into a future without blocking the executor.
  This is useful when calling external libraries that block the thread.

* You should not think of tasks as OS threads. They do not map 1 to 1 and most executors will allow many tasks to run on a single OS thread. This creates multiple gotchas:
  * For instance, using `std::sync::mutex` in an `async` runtime is very dangerous. When you lock the mutex in a thread then yield the executor using `.await` the thread might try to lock the mutex once more in a different task. Hence, prefer `async` alternatives like `tokio::sync::mutex`.
  * Thread-local storage should also be used with care in async contexts as it doesn't map to specific tasks.
  * Device drivers sometimes map to specific OS threads (for instance CUDA.) Prefer `tokio::task::spawn_blocking` when dealing with those.
  * Some C libraries rely on thread local storage as well.

</details>
