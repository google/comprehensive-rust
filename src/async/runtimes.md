# Runtimes and Tasks

A *runtime* provides support for performing operations asynchronously (a
*reactor*) and is responsible for executing futures (an *executor*). Rust does not have a
"built-in" runtime, but several options are available:

 * [Tokio](https://tokio.rs/) - performant, with a well-developed ecosystem of
   functionality like [Hyper](https://hyper.rs/) for HTTP or
   [Tonic](https://github.com/hyperium/tonic) for gRPC.
 * [async-std](https://async.rs/) - aims to be a "std for async", and includes a
   basic runtime in `async::task`.
 * [smol](https://docs.rs/smol/latest/smol/) - simple and lightweight

Several larger applications have their own runtimes. For example,
[Fuchsia](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/lib/fuchsia-async/src/lib.rs)
already has one.

## Tasks

All of these runtimes have the concept of a "Task", similar to a thread but much
less resource-intensive.

A Task has a single top-level Future which the executor polls to make progress.
That future may have one or more nested futures that its `poll` method polls,
corresponding loosely to a call stack. Concurrency is possible within a task by
polling multiple child futures, such as racing a timer and an I/O operation.

```rust,editable,compile_fail
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

Note that of the listed runtimes, only Tokio is supported in the Rust
playground. The playground also does not permit any I/O, so most interesting
async things can't run in the playground.

Copy this example into your prepared `src/main.rs` and run it from there.

Ask students to visualize what the state of the example server would be with a
few connected clients. What tasks exist? What are their Futures?

Refactor the async block into a function, and improve the error handling using `?`.

</details>
