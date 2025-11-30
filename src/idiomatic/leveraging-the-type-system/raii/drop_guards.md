# Drop Guards

A **drop guard** in Rust is a temporary object that performs some kind of
cleanup when it goes out of scope. In the case of `Mutex`, the `lock` method
returns a `MutexGuard` that automatically unlocks the mutex on `drop`:

```rust
struct Mutex {
    is_locked: bool,
}

struct MutexGuard<'a> {
    mutex: &'a mut Mutex,
}

impl Mutex {
    fn new() -> Self {
        Self { is_locked: false }
    }

    fn lock(&mut self) -> MutexGuard<'_> {
        self.is_locked = true;
        MutexGuard { mutex: self }
    }
}

impl Drop for MutexGuard<'_> {
    fn drop(&mut self) {
        self.mutex.is_locked = false;
    }
}
```

<details>

- The example above shows a simplified `Mutex` and its associated guard. Even
  though it is not a production-ready implementation, it illustrates the core
  idea: the guard enforces exclusive access, and its `Drop` implementation is
  used to unlock it again when the guard goes out of scope or is manually
  dropped.

## More to Explore

This example shows a C++ style mutex that does not contain the data it protects.
While this is non idiomatic in Rust, the goal here is only to illustrate the
core idea of a drop guard, not to demonstrate a proper Rust mutex design.

For brevity, several features are omitted:

- Unlike the std `Mutex`, which owns its value, this version keeps the value
  next to the `Mutex` rather than inside it.
- Ergonomic access via `Deref` and `DerefMut` on `MutexGuard`.
- A fully blocking `.lock()` method and a non blocking `try_lock` variant.

You can explore the
[`Mutex` implementation in Rust’s std library](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
as an example of a production ready mutex. The
[`Mutex` from the `parking_lot` crate](https://docs.rs/parking_lot/latest/parking_lot/type.Mutex.html)
is another worthwhile reference.

</details>
