# Broadcast Chat Application

In this exercise, we want to use our new knowledge to implement a broadcast
chat application. We have a chat server that the clients connect to and publish
their messages. The client reads user messages from the standard input, and
sends them to the server. The chat server broadcasts each message that it
receives to all the clients.

For this, we use [a broadcast channel][1] on the server, and
[`tokio_websockets`][2] for the communication between the client and the
server.

Create a new Cargo project and add the following dependencies:

`Cargo.toml`:

<!-- File Cargo.toml -->

```toml
[package]
name = "chat-async"
version = "0.1.0"
edition = "2021"

[dependencies]
futures-util = "0.3.28"
http = "0.2.9"
thiserror = "1.0.40"
tokio = { version = "1.28.1", features = ["net", "macros", "time", "rt", "rt-multi-thread", "io-std", "io-util"] }
tokio-websockets = "0.3.0"
```

## The required APIs
You are going to need the following functions from `tokio` and
[`tokio_websockets`][2]. Spend a few minutes to familiarize yourself with the
API. 

- [WebsocketStream][3]::[next()][4]: for asynchronously reading messages from
  a Websocket Stream.
- [SinkExt][5]::[send()][6] implemented by `WebsocketStream`: for
  asynchronously sending messages on a Websocket Stream.
- [BufReader::read_line][7]: for asynchronously reading user messages
  from the standard input.
- [Sender][8]::[subscribe()][9]: for subscribing to a broadcast channel.


## Two binaries

Normally in a Cargo project, you can have only one binary, and one
`src/main.rs` file. In this project, we need two binaries. One for the client,
and one for the server. You could potentially make them two separate Cargo
projects, but we are going to put them in a single Cargo project with two
binaries. For this to work, the client and server code should go under
`src/bin` (see the [documentation][10]). 

Copy the following server and client code into `src/bin/server.rs` and
`src/bin/client.rs`, respectively. Your task is to complete these files as
described below.
 

`src/bin/server.rs`:

<!-- File src/bin/server.rs -->

```rust,compile_fail
{{#include chat-async/src/bin/server.rs:setup}}

{{#include chat-async/src/bin/server.rs:handle_connection}}

    // TODO: For a hint, see the description of the task below.

{{#include chat-async/src/bin/server.rs:main}}
```

`src/bin/client.rs`:

<!-- File src/bin/client.rs -->

```rust,compile_fail
{{#include chat-async/src/bin/client.rs:setup}}

    // TODO: For a hint, see the description of the task below.

}
```

## Running the binaries
Run the server with:

```shell
$ cargo run --bin server
```

and the client with:

```shell
$ cargo run --bin client
```

## Tasks

* Implement the `handle_connection` function in `server.rs`.
  * Hint: Use `tokio::select!` for concurrently performing two tasks in a
    continuous loop. One task receives messages from the client and broadcasts
    them. The other sends messages received by the server to the client.
* Complete the main function in `client.rs`.
  * Hint: As before, use `tokio::select!` in a continuous loop for concurrently
    performing two tasks: (1) reading user messages from standard input and
    sending them to the server, and (2) receiving messages from the server, and
    displaying them for the user.
* Optional: Once you are done, change the code to broadcast messages to all
  clients, but the sender of the message.

[1]: https://docs.rs/tokio/latest/tokio/sync/broadcast/fn.channel.html
[2]: https://docs.rs/tokio-websockets/0.3.2/tokio_websockets/
[3]: https://docs.rs/tokio-websockets/0.3.2/tokio_websockets/proto/struct.WebsocketStream.html
[4]: https://docs.rs/tokio-websockets/0.3.2/tokio_websockets/proto/struct.WebsocketStream.html#method.next
[5]: https://docs.rs/futures-util/0.3.28/futures_util/sink/trait.SinkExt.html
[6]: https://docs.rs/futures-util/0.3.28/futures_util/sink/trait.SinkExt.html#method.send
[7]: https://docs.rs/tokio/latest/tokio/io/trait.AsyncBufReadExt.html#method.read_line
[8]: https://docs.rs/tokio/latest/tokio/sync/broadcast/struct.Sender.html
[9]: https://docs.rs/tokio/latest/tokio/sync/broadcast/struct.Sender.html#method.subscribe
[10]: https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries
