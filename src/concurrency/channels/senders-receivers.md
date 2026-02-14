---
minutes: 9
---

<!--
Copyright 2024 Google LLC
SPDX-License-Identifier: CC-BY-4.0
-->

# Senders and Receivers

Rust channels have two parts: a [`Sender<T>`] and a [`Receiver<T>`]. The two
parts are connected via the channel, but you only see the end-points.

```rust,editable
# // Copyright 2024 Google LLC
# // SPDX-License-Identifier: Apache-2.0
#
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

- [`mpsc`] stands for Multi-Producer, Single-Consumer. `Sender` and `SyncSender`
  implement `Clone` (so you can make multiple producers) but `Receiver` does
  not.
- [`send()`] and [`recv()`] return `Result`. If they return `Err`, it means the
  counterpart `Sender` or `Receiver` is dropped and the channel is closed.

</details>

[`Sender<T>`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html
[`Receiver<T>`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html
[`send()`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send
[`recv()`]: https://doc.rust-lang.org/std/sync/mpsc/struct.Receiver.html#method.recv
[`mpsc`]: https://doc.rust-lang.org/std/sync/mpsc/index.html
