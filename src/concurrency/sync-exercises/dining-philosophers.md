---
minutes: 20
---

# Dining Philosophers

The dining philosophers problem is a classic problem in concurrency:

> Five philosophers dine together at the same table. Each philosopher has their
> own place at the table. There is a chopstick between each plate. The dish
> served is spaghetti which requires two chopsticks to eat. Each philosopher can
> only alternately think and eat. Moreover, a philosopher can only eat their
> spaghetti when they have both a left and right chopstick. Thus two chopsticks
> will only be available when their two nearest neighbors are thinking, not
> eating. After an individual philosopher finishes eating, they will put down
> both chopsticks.

You will need a local [Cargo installation](../../cargo/running-locally.md) for
this exercise. Copy the code below to a file called `src/main.rs`, fill out the
blanks, and test that `cargo run` does not deadlock:

<!-- File src/main.rs -->

```rust,compile_fail
{{#include dining-philosophers.rs:Philosopher}}
    // left_chopstick: ...
    // right_chopstick: ...
    // thoughts: ...
}

{{#include dining-philosophers.rs:Philosopher-think}}

{{#include dining-philosophers.rs:Philosopher-eat}}
        // Pick up chopsticks...
{{#include dining-philosophers.rs:Philosopher-eat-end}}
    // Create chopsticks

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
edition = "2024"
```

<details>

- Encourage students to focus first on implementing a solution that "mostly"
  works.
- The deadlock in the simplest solution is a general concurrency problem and
  highlights that Rust does not automatically prevent this sort of bug.

</details>
