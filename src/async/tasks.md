# Tasks

Rust has a task system, which is a form of lightweight threading.

A task has a single top-level future which the executor polls to make progress.
That future may have one or more nested futures that its `poll` method polls,
corresponding loosely to a call stack. Concurrency within a task is possible by
polling multiple child futures, such as racing a timer and an I/O operation.

```rust,compile_fail
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;
	println!("listening on port 6142");

    loop {
        let (mut socket, addr) = listener.accept().await?;

        println!("connection from {addr:?}");

        tokio::spawn(async move {
            if let Err(e) = socket.write_all(b"Who are you?\n").await {
                println!("socket error: {e:?}");
                return;
            }

            let mut buf = vec![0; 1024];
            let reply = match socket.read(&mut buf).await {
                Ok(n) => {
                    let name = std::str::from_utf8(&buf[..n]).unwrap().trim();
                    format!("Thanks for dialing in, {name}!\n")
                }
                Err(e) => {
                    println!("socket error: {e:?}");
                    return;
                }
            };

            if let Err(e) = socket.write_all(reply.as_bytes()).await {
                println!("socket error: {e:?}");
            }
        });
    }
}
```

<details>

Copy this example into your prepared `src/main.rs` and run it from there.

* Ask students to visualize what the state of the example server would be with a
  few connected clients. What tasks exist? What are their Futures?

* This is the first time we've seen an `async` block. This is similar to a
  closure, but does not take any arguments. Its return value is a Future,
  similar to an `async fn`. 

* Refactor the async block into a function, and improve the error handling using `?`.

</details>
