# Cancellation

Dropping a future implies it can never be polled again. This is called *cancellation*.
It can occur at any `await` point. Care is needed with stateful resources.

```rust,editable,compile_fail
use std::time::Duration;
use tokio::io::{DuplexStream, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::time;

// A producer which slowly writes bytes
async fn producer(mut stream: DuplexStream) -> std::io::Result<()> {
    let data = b"hello\nworld\n";
    for chunk in data.chunks(2) {
        stream.write_all(chunk).await?;
        time::sleep(Duration::from_millis(10)).await;
    }
    Ok(())
}

// A consumer which is waiting for one of several events
async fn consumer(stream: DuplexStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut buf = String::new();
    let mut interval = time::interval(Duration::from_millis(5));
    loop {
        tokio::select! { 
            res = reader.read_line(&mut buf) => match res? {
                0 => return Ok(()),
                _ => {
                    print!("{}", buf);
                    buf.clear()
                }
            },
            _ = interval.tick() => {
                println!("tick!");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (a, b) = tokio::io::duplex(10);
    let h1 = tokio::spawn(producer(a));
    let h2 = tokio::spawn(consumer(b));
    h1.await.unwrap().unwrap();
    h2.await.unwrap().unwrap();
}
```

<details>

* [`AsyncBufReadExt::read_line`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line)
  isn't cancellation safe. It reads data, but only guarantees that it appears in `buf` once there's a newline.

    * `interval` sometimes ticks before a full line is received and `select!`
      drops the `read_line` future.

    * Use `AsyncBufReadExt::lines` to convert the reader into a stream of lines,
      which has a cancellation-safe `next_line` method.

        ```rust,compile_fail
        let mut lines = BufReader::new(stream).lines();
        loop {
            select! {
                res = lines.next_line() => match res? {
                    None => return Ok(()),
                    Some(l) => {
                        println!("{}", l);
                    }
                },
              ..
            }
        }
        ```

* [`Interval::tick`](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html#method.tick)
  is cancellation safe because it keeps track of whether a tick has been 'delivered'.

* Graceful shutdown, i.e. stopping based on some signal, is sometimes an
  alternative to cancellation.

    * For example, a task receiving messages from an mpsc channel might
      do some cleanup when it detects that all senders have been dropped.
    
    * [tokio-util](https://docs.rs/tokio-util/latest/tokio_util/) inculdes a
      [`CancellationToken`](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
      for explicitly signaling when tasks should shut down.

</details>
