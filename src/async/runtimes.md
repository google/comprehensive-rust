# Runtimes

A _runtime_ provides support for performing operations asynchronously (a
_reactor_) and is responsible for executing futures (an _executor_). Rust does
not have a "built-in" runtime, but several options are available:

- [Tokio](https://tokio.rs/): performant, with a well-developed ecosystem of
  functionality like [Hyper](https://hyper.rs/) for HTTP or
  [Tonic](https://github.com/hyperium/tonic) for gRPC.
- [async-std](https://async.rs/): aims to be a "std for async", and includes a
  basic runtime in `async::task`.
- [smol](https://docs.rs/smol/latest/smol/): simple and lightweight

Several larger applications have their own runtimes. For example,
[Fuchsia](https://fuchsia.googlesource.com/fuchsia/+/refs/heads/main/src/lib/fuchsia-async/src/lib.rs)
already has one.

<details>

- Note that of the listed runtimes, only Tokio is supported in the Rust
  playground. The playground also does not permit any I/O, so most interesting
  async things can't run in the playground.

- Futures are "inert" in that they do not do anything (not even start an I/O
  operation) unless there is an executor polling them. This differs from JS
  Promises, for example, which will run to completion even if they are never
  used.

</details>
