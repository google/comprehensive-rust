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

- The channel is called asynchronous because there is no synchronization between
  sending and receiving.
- The channel buffers the values. The buffer grows automatically, similar to how
  a `Vec` grows when you push data to it.
- The channel takes ownership of the values when you call [`send()`]. This is
  seen in the signature: it takes `T` by value. You thus lose access to the
  value you send into a channel.

</details>

[`mpsc::channel()`]: https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html
[`send()`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send
