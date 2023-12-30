# Cancellation

Dropping a future implies it can never be polled again. This is called
_cancellation_ and it can occur at any `await` point. Care is needed to ensure
the system works correctly even when futures are cancelled. For example, it
shouldn't deadlock or lose data.

```rust,editable,compile_fail
use std::io::{self, ErrorKind};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, DuplexStream};

struct LinesReader {
    stream: DuplexStream,
}

impl LinesReader {
    fn new(stream: DuplexStream) -> Self {
        Self { stream }
    }

    async fn next(&mut self) -> io::Result<Option<String>> {
        let mut bytes = Vec::new();
        let mut buf = [0];
        while self.stream.read(&mut buf[..]).await? != 0 {
            bytes.push(buf[0]);
            if buf[0] == b'\n' {
                break;
            }
        }
        if bytes.is_empty() {
            return Ok(None);
        }
        let s = String::from_utf8(bytes)
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "not UTF-8"))?;
        Ok(Some(s))
    }
}

async fn slow_copy(source: String, mut dest: DuplexStream) -> std::io::Result<()> {
    for b in source.bytes() {
        dest.write_u8(b).await?;
        tokio::time::sleep(Duration::from_millis(10)).await
    }
    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (client, server) = tokio::io::duplex(5);
    let handle = tokio::spawn(slow_copy("hi\nthere\n".to_owned(), client));

    let mut lines = LinesReader::new(server);
    let mut interval = tokio::time::interval(Duration::from_millis(60));
    loop {
        tokio::select! {
            _ = interval.tick() => println!("tick!"),
            line = lines.next() => if let Some(l) = line? {
                print!("{}", l)
            } else {
                break
            },
        }
    }
    handle.await.unwrap()?;
    Ok(())
}
```

<details>

- The compiler doesn't help with cancellation-safety. You need to read API
  documentation and consider what state your `async fn` holds.

- Unlike `panic` and `?`, cancellation is part of normal control flow (vs
  error-handling).

- The example loses parts of the string.

  - Whenever the `tick()` branch finishes first, `next()` and its `buf` are
    dropped.

  - `LinesReader` can be made cancellation-safe by making `buf` part of the
    struct:
    ```rust,compile_fail
    struct LinesReader {
        stream: DuplexStream,
        bytes: Vec<u8>,
        buf: [u8; 1],
    }

    impl LinesReader {
        fn new(stream: DuplexStream) -> Self {
            Self { stream, bytes: Vec::new(), buf: [0] }
        }
        async fn next(&mut self) -> io::Result<Option<String>> {
            // prefix buf and bytes with self.
            // ...
            let raw = std::mem::take(&mut self.bytes);
            let s = String::from_utf8(raw)
            // ...
        }
    }
    ```

- [`Interval::tick`](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html#method.tick)
  is cancellation-safe because it keeps track of whether a tick has been
  'delivered'.

- [`AsyncReadExt::read`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read)
  is cancellation-safe because it either returns or doesn't read data.

- [`AsyncBufReadExt::read_line`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line)
  is similar to the example and _isn't_ cancellation-safe. See its documentation
  for details and alternatives.

</details>
