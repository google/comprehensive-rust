# Pitfalls of async/await

Most async runtimes only allow IO tasks to run concurrently.
This means that CPU blocking tasks will block the executor and prevent other tasks from being executed.
An easy workaround is to use async equivalent methods where possible.

```rust,editable,compile_fail
use futures::future::join_all;
use std::time::Instant;

async fn std_sleep_ms(duration_ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(duration_ms));
}

async fn tokio_sleep_ms(duration_ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(duration_ms));
}

// Single threaded executor for better reproducibility in runtime.
#[tokio::main(flavor = "current_thread")]
async fn main() {
    let std_sleep_futures = (1..=100).into_iter().map(std_sleep_ms);
    let tokio_sleep_futures = (1..=100).into_iter().map(tokio_sleep_ms);

    let now = Instant::now();
    join_all(std_sleep_futures).await;
    assert!(now.elapsed().as_millis() >= 5050);

    let now = Instant::now();
    join_all(tokio_sleep_futures).await;
    let runtime = now.elapsed().as_millis();
    assert!(runtime < 150 && runtime >= 100);
}
```

Using `std::sync::mutex` in an `async` runtime is also very dangerous. When you lock the mutex in a thread then yield the executor using `.await` the thread might try to lock the mutex once more in a different task.
Hence, prefer `async` alternatives like `tokio::sync::mutex`.

```rust,editable,compile_fail
use futures::future::join_all;
use std::sync::Arc;

async fn increment_tokio_mutex(mutex: Arc<tokio::sync::Mutex<i32>>) {
    let mut guard = mutex.lock().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
    *guard += 1;
}

async fn increment_std_mutex(mutex: Arc<std::sync::Mutex<i32>>) {
    let mut guard = mutex.lock().expect("Failed to unlock std mutex.");
    // Yield executor, another task in the same thread might try to lock again before we unlock.
    tokio::time::sleep(tokio::time::Duration::from_millis(0)).await;
    *guard += 1;
}

// Multi threaded runtime
#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let tokio_mutex = Arc::new(tokio::sync::Mutex::new(0i32));
    let tokio_futures = (1..=100)
        .into_iter()
        .map(|_| increment_tokio_mutex(Arc::clone(&tokio_mutex)));
    join_all(tokio_futures).await;
    assert_eq!(*tokio_mutex.lock().await, 100);

    let std_mutex = Arc::new(std::sync::Mutex::new(0i32));
    let std_futures = (1..=100)
        .into_iter()
        .map(|_| increment_std_mutex(Arc::clone(&std_mutex)));
    join_all(std_futures).await; // <- Deadlock
    assert_eq!(*std_mutex.lock().expect("Failed to unlock std mutex."), 100);
}
```

<details>

When there are no async alternatives or you have a CPU-heavy task as part of your compute graph, use a std::thread. `tokio` has `tokio::task::spawn_blocking` to spawn a CPU-bound task as part of its threadpool.

</details>
