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

* Threads are all daemon threads, the main thread does not wait for them.
* Thread panics are independent of each other.
  * Panics can carry a payload, which can be unpacked with `downcast_ref`.
