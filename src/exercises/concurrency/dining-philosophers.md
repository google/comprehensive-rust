# Dining Philosophers

The dining philosophers problem is a classic problem in concurrency:

> Five philosophers dine together at the same table. Each philosopher has their
> own place at the table. There is a fork between each plate. The dish served is
> a kind of spaghetti which has to be eaten with two forks. Each philosopher can
> only alternately think and eat. Moreover, a philosopher can only eat their
> spaghetti when they have both a left and right fork. Thus two forks will only
> be available when their two nearest neighbors are thinking, not eating. After
> an individual philosopher finishes eating, they will put down both forks.

You will need a local [Cargo installation](../../cargo/running-locally.md) for
this exercise. Copy the code below to a file called `src/main.rs`, fill out the
blanks, and test that `cargo run` does not deadlock:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include dining-philosophers.rs:Philosopher}}
    // left_fork: ...
    // right_fork: ...
    // thoughts: ...
}

{{#include dining-philosophers.rs:Philosopher-think}}

{{#include dining-philosophers.rs:Philosopher-eat}}
        // Pick up forks...
{{#include dining-philosophers.rs:Philosopher-eat-end}}
    // Create forks

    // Create philosophers

    // Make each of them think and eat 100 times

    // Output their thoughts
}
```

You can use the following `Cargo.toml`:

<!-- File Cargo.toml -->

```toml
[package]
name = "dining-philosophers"
version = "0.1.0"
edition = "2021"
```
