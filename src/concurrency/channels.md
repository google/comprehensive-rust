# Channels

Rust channels have two parts: a `Sender<T>` and a `Receiver<T>`. The two parts
are connected via the channel, but you only see the end-points.

```rust,editable
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());
}
```

<details>

- `mpsc` stands for Multi-Producer, Single-Consumer. `Sender` and `SyncSender`
  implement `Clone` (so you can make multiple producers) but `Receiver` does
  not.
- `send()` and `recv()` return `Result`. If they return `Err`, it means the
  counterpart `Sender` or `Receiver` is dropped and the channel is closed.

</details>
