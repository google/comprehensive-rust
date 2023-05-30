# Cancellation

Dropping a future implies it can never be polled again. This is called *cancellation*
and it can occur at any `await` point. Care is needed to ensure the system works
correctly even when futures are cancelled. For example, it shouldn't deadlock or
lose data.

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
        let mut buf = Vec::new();
        loop {
            buf.push(0);
            let last = buf.len() - 1;
            if self.stream.read(&mut buf[last..]).await? == 0 {
                // Assume file ends with newline
                return Ok(None)
            }
            if buf[last] == b'\n' {
                break;
            }
        }
        let s = String::from_utf8(buf)
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

* The compiler doesn't help with cancellation-safety. You need to read API
  documentation and consider what state your `async fn` holds.

* Cancellation can be compared to panics and the `?` operator

    * Unlike `panic` and `?`, cancellation is part of normal control flow
      (vs error-handling).

* The example loses parts of the string.

    * Whenever the `tick()` branch finishes first, `next()` and its `buf` are dropped.

    * `LinesReader` can be made cancellation-safe by makeing `buf` part of the struct:
        ```rust,compile_fail
        struct LinesReader {
            stream: DuplexStream,
            buf: Vec<u8>,
        }

        impl LinesReader {
            fn new(stream: DuplexStream) -> Self {
                Self { stream, buf: Vec::new() }
            }
            async fn next(&mut self) -> io::Result<Option<String>> {
                // replace buf with self.buf
                // ...
                let raw = std::mem::take(&mut self.buf);
                let s = String::from_utf8(raw)
                // ...
            }
        }
        ```

* [`Interval::tick`](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html#method.tick)
  is cancellation-safe because it keeps track of whether a tick has been 'delivered'.

* [`AsyncReadExt::read`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncReadExt.html#method.read)
  is cancellation-safe because it either returns or doesn't read data.

* [`AsyncBufReadExt::read_line`](https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line)
  is similar to the example and *isn't* cancellation-safe. See its documentation
  for details and alternatives.

</details>
