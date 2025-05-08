---
minutes: 8
---

# Async

The kernel performs many operations concurrently and involves significant
amounts of interaction between CPU cores and other devices. For this reason, it
would be no surprise to see that async Rust would be a fundamental requirement
for using Rust in the kernel. But the kernel is central arbiter of most
synchronization and is currently written in regular, synchronous C.

Rust code making use of `async` mostly exists to write composable code that will
run atop event loops, but the Linux kernel is not really organized as an event
loop: user tasks call directly into the kernel; control flow for interrupts is
handled by hardware.

As such, `async` support is not critical for most kernel programming tasks.
However, it is possible to view some components of the kernel as async
executors, and some work has been done in this direction. Wedson Almeida Filho
implemented both workqueue-based[^1] and single-threaded async executors as
proofs of concept.

There is not a fundamental incompatibility between Rust-for-Linux and Rust
`async`, which is a similar situation to the amenability of `async` to use in
embedded Rust programming (e.g. the Embassy project).

Nonetheless, no killer application of `async` in Rust for Linux has made it a
priority.

<details>

[^1]: <https://github.com/Rust-for-Linux/linux/tree/rust/rust/kernel/kasync>

An example of an async server using the kernel async executor may be found
[here](https://github.com/Rust-for-Linux/linux/blob/rust/samples/rust/rust_echo_server.rs).

</details>
