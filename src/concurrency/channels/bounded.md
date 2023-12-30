# Bounded Channels

With bounded (synchronous) channels, `send` can block the current thread:

```rust,editable
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 1..10 {
            tx.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: sent Message {i}");
        }
        println!("{thread_id:?}: done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx.iter() {
        println!("Main: got {msg}");
    }
}
```

<details>

- Calling `send` will block the current thread until there is space in the
  channel for the new message. The thread can be blocked indefinitely if there
  is nobody who reads from the channel.
- A call to `send` will abort with an error (that is why it returns `Result`) if
  the channel is closed. A channel is closed when the receiver is dropped.
- A bounded channel with a size of zero is called a "rendezvous channel". Every
  send will block the current thread until another thread calls `read`.

</details>
