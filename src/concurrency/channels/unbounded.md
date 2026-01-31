---
minutes: 2
---

# Unbounded Channels

You get an unbounded and asynchronous channel with [`mpsc::channel()`]:

```rust,editable
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx {
        println!("Main: got {msg}");
    }
}
```

[`mpsc::channel()`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html

<details>

- An unbounded channel will allocate as much space as is necessary to store
  pending messages. The `send()` method will not block the calling thread.
- A call to `send()` will abort with an error (that is why it returns `Result`)
  if the channel is closed. A channel is closed when the receiver is dropped.

</details>
