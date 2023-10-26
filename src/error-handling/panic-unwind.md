# Catching the Stack Unwinding

By default, a panic will cause the stack to unwind. The unwinding can be caught:

```rust,editable
use std::panic;

fn main() {
    let result = panic::catch_unwind(|| {
        "No problem here!"
    });
    println!("{result:?}");

    let result = panic::catch_unwind(|| {
        panic!("oh no!");
    });
    println!("{result:?}");
}
```

- This can be useful in servers which should keep running even if a single
  request crashes.
- This does not work if `panic = 'abort'` is set in your `Cargo.toml`.
