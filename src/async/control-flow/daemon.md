# Daemon

Tasks can be spawned without having to be awaited. They will be scheduled like any other tasks by the executor but won't block any running task. This can be useful for tasks that function like actors, receiving messages and sending messages to other tasks through channels. It can also be useful to log metrics or ping system's health.

```rust,editable,compile_fail

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let seconds_since_beginning = Arc::new(AtomicUsize::from(0));
    let counter = Arc::clone(&seconds_since_beginning);
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(1)).await;
            counter.fetch_add(1, Ordering::SeqCst);
        }
    });

    sleep(Duration::from_millis(4500)).await;
    assert_eq!(seconds_since_beginning.load(Ordering::Relaxed), 4);
}


```

<details>

* It is good practice to make your deamons exit because some other blocking task might depend on them. Which would prevent your main thread from ever closing. You can use a `oneshot` channel to signal the task to terminate. You can also use the `ctrl+c` signal handler from `tokio` as an interrupt signal.

</details>
