# Logging

It would be nice to be able to use the logging macros from the [`log`][1] crate.
We can do this by implementing the `Log` trait.

```rust,editable,compile_fail
{{#include examples/src/logger.rs:main}}
```

<details>

- The unwrap in `log` is safe because we initialise `LOGGER` before calling
  `set_logger`.

</details>

[1]: https://crates.io/crates/log
