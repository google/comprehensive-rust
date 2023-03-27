# Reentering threads

Async tasks do not map to os threads. This means that multiple tasks can be scheduled on the same os thread one after the other without being aware of one another. While this is great for performance, it also comes with some caveats that can cause problems that are hard to debug.


For instance, using `std::sync::mutex` in an `async` runtime is very dangerous. When you lock the mutex in a thread then yield the executor using `.await` the thread might try to lock the mutex once more in a different task.
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

- Thread-local storage should also be used with care in async contexts as it doesn't map to specific tasks.
- Device drivers sometimes map to specific OS threads (for instance CUDA.) Prefer `tokio::task::spawn_blocking` when dealing with those.
- Some C libraries rely on thread local storage as well.

</details>
