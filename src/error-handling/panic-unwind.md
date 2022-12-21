# Catching the Stack Unwinding

By default, a panic will cause the stack to unwind. The unwinding can be caught:

```rust
use std::panic;

let result = panic::catch_unwind(|| {
    println!("hello!");
});
assert!(result.is_ok());

let result = panic::catch_unwind(|| {
    panic!("oh no!");
});
assert!(result.is_err());
```

* This can be useful in servers which should keep running even if a single
  request crashes.
* This does not work if `panic = abort` is set in your `Cargo.toml`.
