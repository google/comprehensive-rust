# Mutex and MutexGuard

In earlier examples, RAII was used to manage concrete resources like file
descriptors. With a `Mutex`, the "resource" is mutable access to a value.
You access the value by calling `lock`, which then returns a `MutexGuard`
which will unlock the `Mutex` automatically when dropped.

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(vec![1, 2, 3]);

    let mut guard = m.lock().unwrap();
    guard.push(4);
    guard.push(5);
    println!("{guard:?}");
}
```

<details>

- A `Mutex` controls exclusive access to a value. Unlike earlier RAII examples,
  the resource here is not external but logical: the right to mutate shared
  data.

- This right is represented by a `MutexGuard`. Only one can exist at a time.
  While it lives, it provides `&mut T` access.

- Although `lock()` takes `&self`, it returns a `MutexGuard` with mutable
  access. This is possible through interior mutability: a common pattern for
  safe shared-state mutation.

- `MutexGuard` implements `Deref` and `DerefMut`, making access ergonomic. You
  lock the mutex, use the guard like a `&mut T`, and the lock is released
  automatically when the guard goes out of scope.

- The release is handled by `Drop`. There is no need to call a separate unlock
  function — this is RAII in action.

</details>
