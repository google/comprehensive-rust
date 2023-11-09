---
minutes: 3
existing course material:
- error-handling/panics.md
- error-handling.md
- error-handling/panic-unwind.md
---

<!-- NOTES:
Students only need to know that it's possible, but unusual, to catch panics
-->
# Panics

# Panics

Rust will trigger a panic if a fatal error happens at runtime:

```rust,editable,should_panic
fn main() {
    let v = vec![10, 20, 30];
    println!("v[100]: {}", v[100]);
}
```

* Panics are for unrecoverable and unexpected errors.
  * Panics are symptoms of bugs in the program.
* Use non-panicking APIs (such as `Vec::get`) if crashing is not acceptable.
# Error Handling

Error handling in Rust is done using explicit control flow:

* Functions that can have errors list this in their return type,
* There are no exceptions.

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
