# Mutex

In earlier examples, RAII was used to manage concrete resources like file
descriptors. With a `Mutex`, the resource is more abstract: exclusive access to
a value.

Rust models this using a `MutexGuard`, which ties access to a critical section
to the lifetime of a value on the stack.

```rust
#[derive(Debug)]
struct Mutex<T> {
    value: std::cell::UnsafeCell<T>,
    // [...]
}

#[derive(Debug)]
struct MutexGuard<'a, T> {
    value: &'a mut T,
    // [...]
}

impl<T> Mutex<T> {
    fn new(value: T) -> Self {
        Self {
            value: std::cell::UnsafeCell::new(value),
            // [...]
        }
    }

    fn lock(&self) -> MutexGuard<T> {
        // [...]
        let value = unsafe { &mut *self.value.get() };
        MutexGuard { value }
    }
}

impl<'a, T> std::ops::Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.value
    }
}

impl<'a, T> std::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.value
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        // [...]
        println!("drop MutexGuard");
    }
}

fn main() {
    let m = Mutex::new(vec![1, 2, 3]);

    let mut guard = m.lock();
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

- See this example showing the standard library API with poisoning:
  <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=6fb0c2e9e5cbcbbae1c664f4650b8c92>

### Mutex Lock Lifecycle

```bob
+---------------+         +----------------------+
|  Mutex<T>     |  lock   |   MutexGuard<T>      |
| ( Unlocked )  +-------->| ( Exclusive Access ) |
+---------------+         +-------+--------------+
                                  |
                                  | drop
                                  v
+---------------+  yes  +-------------------+
|  Mutex<T>     |<------+ Thread panicking? |
| ( Poisoned )  |       +-------+-----------+
+------+--------+               | no
      |                         v
      |                  +---------------+
      | lock             |  Mutex<T>     |
      |                  | ( Unlocked )  |
      |                  +---------------+
      v
+------------------+
| Err ( Poisoned ) |
+------------------+
```

</details>
