# Mutex and MutexGuard

In earlier examples, RAII was used to manage concrete resources like file
descriptors. With a `Mutex`, the resource is more abstract: exclusive access to
a value.

Rust models this using a `MutexGuard`, which ties access to a critical section
to the lifetime of a value on the stack.

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
  While it lives, it provides `&mut T` access — enforced using `UnsafeCell`.

- Although `lock()` takes `&self`, it returns a `MutexGuard` with mutable
  access. This is possible through interior mutability: a common pattern for
  safe shared-state mutation.

- `MutexGuard` implements `Deref` and `DerefMut`, making access ergonomic. You
  lock the mutex, use the guard like a `&mut T`, and the lock is released
  automatically when the guard goes out of scope.

- The release is handled by `Drop`. There is no need to call a separate unlock
  function — this is RAII in action.

## Poisoning

- If a thread panics while holding the lock, the value may be in a corrupt
  state.

- To signal this, the standard library uses poisoning. When `Drop` runs during a
  panic, the mutex marks itself as poisoned.

- On the next `lock()`, this shows up as an error. The caller must decide
  whether to proceed or handle the error differently.

</details>
