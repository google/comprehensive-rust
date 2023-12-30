# Threads

Rust threads work similarly to threads in other languages:

```rust,editable
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Count in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 1..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }
}
```

- Threads are all daemon threads, the main thread does not wait for them.
- Thread panics are independent of each other.
  - Panics can carry a payload, which can be unpacked with `downcast_ref`.

<details>

Key points:

- Notice that the thread is stopped before it reaches 10 --- the main thread is
  not waiting.

- Use `let handle = thread::spawn(...)` and later `handle.join()` to wait for
  the thread to finish.

- Trigger a panic in the thread, notice how this doesn't affect `main`.

- Use the `Result` return value from `handle.join()` to get access to the panic
  payload. This is a good time to talk about [`Any`].

[`Any`]: https://doc.rust-lang.org/std/any/index.html

</details>
