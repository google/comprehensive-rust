# Drop Guards

A **drop guard** in Rust is a temporary object that performs some kind of
cleanup when it goes out of scope. In the case of `Mutex`, the `lock` method
returns a `MutexGuard` that automatically unlocks the mutex on drop:

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

</details>
