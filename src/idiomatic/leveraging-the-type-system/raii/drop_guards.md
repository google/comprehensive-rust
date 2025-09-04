# Drop Guards

A **drop guard** in Rust is a temporary _RAII_ guard that executes a specific
action when it goes out of scope.

It acts as a wrapper around a value, ensuring that some cleanup or secondary
behavior happens automatically when the guard is dropped.

One of the most common examples is `MutexGuard`, which represents temporary
exclusive access to a shared resource.

```rust
#[derive(Debug)]
struct Mutex<T> {
    value: std::cell::UnsafeCell<T>,
    is_locked: std::sync::atomic::AtomicBool,
}

#[derive(Debug)]
struct MutexGuard<'a, T> {
    value: &'a mut T,
    mutex: &'a Mutex<T>,
}

impl<T> Mutex<T> {
    fn new(value: T) -> Self {
        Self {
            value: std::cell::UnsafeCell::new(value),
            is_locked: std::sync::atomic::AtomicBool::new(false),
        }
    }

    fn lock(&self) -> MutexGuard<'_, T> {
        // Acquire the lock and create the guard object.
        if self.is_locked.swap(true, std::sync::atomic::Ordering::AcqRel) {
            todo!("Block until the lock is released");
        }
        let value = unsafe { &mut *self.value.get() };
        MutexGuard { value, mutex: self }
    }
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.is_locked.store(false, std::sync::atomic::Ordering::Release);
    }
}

fn main() {
    let m = Mutex::new(vec![1, 2, 3]);

    let mut guard = m.lock();
    guard.value.push(4);
    guard.value.push(5);
    println!("{guard:?}");
}
```

<details>

- The example above shows a simplified `Mutex` and its associated guard. Even
  though it is not a production-ready implementation, it illustrates the core
  idea: the guard enforces exclusive access, and its `Drop` implementation
  guarantees that the lock is released when the guard goes out of scope.

- A few things are left out for brevity:

  - `Deref` and `DerefMut` implementations for `MutexGuard`, which would allow
    you to use the guard as if it were a direct reference to the inner value.
  - Making `.lock()` truly blocking, so that it waits until the mutex is free
    before returning.
    - In addition, a `.try_lock()` method could be added to provide a
      non-blocking alternative, returning `Option::None` or `Result::Err(...)`
      if the mutex is still locked.

- Panics are not explicitly handled in the `Drop` implementation here. In
  practice, one can use `std::thread::panicking()` to check if the guard was
  dropped during a panic.

  - The standard library’s `std::sync::Mutex` uses this to implement
    **poisoning**, where a mutex is marked as poisoned if a panic occurs while
    holding the lock, since the protected value may now be in an inconsistent
    state.

</details>
