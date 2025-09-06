---
minutes: 10
---

# Generic Data Types

You can use generics to abstract over the concrete field type. Returning to the
exercise for the previous segment:

```rust,editable
pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

/// Only log messages up to the given verbosity level.
struct VerbosityFilter<L> {
    max_verbosity: u8,
    inner: L,
}

impl<L: Logger> Logger for VerbosityFilter<L> {
    fn log(&self, verbosity: u8, message: &str) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

fn main() {
    let logger = VerbosityFilter { max_verbosity: 3, inner: StderrLogger };
    logger.log(5, "FYI");
    logger.log(2, "Uhoh");
}
```

<details>

- _Q:_ Why is `L` specified twice in `impl<L: Logger> .. VerbosityFilter<L>`?
  Isn't that redundant?
  - This is because it is a generic implementation section for generic type.
    They are independently generic.
  - It means these methods are defined for any `L`.
  - It is possible to write `impl VerbosityFilter<StderrLogger> { .. }`.
    - `VerbosityFilter` is still generic and you can use `VerbosityFilter<f64>`,
      but methods in this block will only be available for
      `VerbosityFilter<StderrLogger>`.
- Note that we don't put a trait bound on the `VerbosityFilter` type itself. You
  can put bounds there as well, but generally in Rust we only put the trait
  bounds on the impl blocks.

</details>
