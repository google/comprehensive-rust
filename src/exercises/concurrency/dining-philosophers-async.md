# Dining Philosophers - Async

See [dining philosophers](dining-philosophers.md) for a description of the
problem.

As before, you will need a local
[Cargo installation](../../cargo/running-locally.md) for this exercise. Copy
the code below to a file called `src/main.rs`, fill out the blanks, and test
that `cargo run` does not deadlock:

<!-- File src/main.rs -->

```rust,compile_fail
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::time;
use tokio::sync::mpsc::{self, Sender};
use tokio::sync::Mutex;


struct Fork;

/// We've already learnt that to avoid a deadlock, we have to break the
/// symmetry. So here we call the forks the first_fork and the second_fork.
/// The left fork is not necessarily the first fork.
struct Philosopher {
    name: String,
    first_fork: ...,
    second_fork: ...,
    thoughts: ...,
}

impl Philosopher {
    async fn think(&self) {
        self.thoughts
            .send(format!("Eureka! {} has a new idea!", &self.name)).await
            .unwrap();
    }

    async fn eat(&self) {
        // Pick up forks...

        println!("{} is eating...", &self.name);
        time::sleep(time::Duration::from_millis(10)).await;
    }
}

static PHILOSOPHERS: &[&str] =
    &["Socrates", "Plato", "Aristotle", "Thales", "Pythagoras"];

#[tokio::main]
async fn main() {
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
provided by the `tokio` crate.
