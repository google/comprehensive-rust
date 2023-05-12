# Dining Philosophers - Async

See [dining philosophers](dining-philosophers.md) for a description of the
problem.

As before, you will need a local
[Cargo installation](../../cargo/running-locally.md) for this exercise. Copy
the code below to a file called `src/main.rs`, fill out the blanks, and test
that `cargo run` does not deadlock:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include dining-philosophers-async.rs:Philosopher}}
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

{{#include dining-philosophers-async.rs:Philosopher-think}}

{{#include dining-philosophers-async.rs:Philosopher-eat}}
{{#include dining-philosophers-async.rs:Philosopher-eat-body}}
{{#include dining-philosophers-async.rs:Philosopher-eat-end}}
    // Create forks

    // Create philosophers

    // Make them think and eat

    // Output their thoughts
}
```

Since this time you are using Async Rust, you'll need a `tokio` dependency.
You can use the following `Cargo.toml`:

<!-- File Cargo.toml -->

```toml
[package]
name = "dining-philosophers-async-dine"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = {version = "1.26.0", features = ["sync", "time", "macros", "rt-multi-thread"]}
```

Also note that this time you have to use the `Mutex` and the `mpsc` module
from the `tokio` crate.

<details>

* Can you make your implementation single-threaded? 

</details>
