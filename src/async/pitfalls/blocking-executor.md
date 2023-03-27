# Blocking the executor

Most async runtimes only allow IO tasks to run concurrently.
This means that CPU blocking tasks will block the executor and prevent other tasks from being executed.
An easy workaround is to use async equivalent methods where possible.

```rust,editable,compile_fail
use futures::future::join_all;
use std::time::Instant;

// Uncomment to try with `spawn_blocking` around `std::thread::sleep`.
const USE_SPAWN_BLOCKING: bool = false;

async fn std_sleep_ms(duration_ms: u64) {
    if USE_SPAWN_BLOCKING {
        tokio::task::spawn_blocking(move || {
            std::thread::sleep(std::time::Duration::from_millis(duration_ms));
        })
        .await
        .unwrap();
    } else {
        std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    }
}

async fn tokio_sleep_ms(duration_ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms)).await;
}

// Single threaded executor for better reproducibility in runtime.
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let std_sleep_futures = (1..=100).map(std_sleep_ms);
    let tokio_sleep_futures = (1..=100).map(tokio_sleep_ms);

    let now = Instant::now();
    join_all(std_sleep_futures).await;
    assert!(now.elapsed().as_millis() >= 5050);

    let now = Instant::now();
    join_all(tokio_sleep_futures).await;
    let runtime = now.elapsed().as_millis();
    assert!((100..150).contains(&runtime));
}

```

<details>

- Using `std::thread::sleep` blocks the thread, so it prevents the executor from running. It means that while all futures are spawned at the same time, they all run one after the other. The runtime is the sum of all the `sleep` times. Try changing the runtime to `multi_thread` in a multi core environment to see how it impacts the run time.
- A simple fix is to use `tokio::time::sleep`. Now, the `sleep` calls are `async` and they are properly scheduled by the executor.
- Another fix would be to `tokio::task::spawn_blocking` which spawns an actual thread and transforms its handle into a future without blocking the executor. This thread is also scheduled as part of the executor's threadpool to grant better performance.

</details>
