---
minutes: 14
---

# `Mutex`

[`Mutex<T>`][1] ensures mutual exclusion _and_ allows mutable access to `T`
behind a read-only interface (another form of
[interior mutability](../../borrowing/interior-mutability.md)):

```rust,editable
use std::sync::Mutex;

fn main() {
    let v = Mutex::new(vec![10, 20, 30]);
    println!("v: {:?}", v.lock().unwrap());

    {
        let mut guard = v.lock().unwrap();
        guard.push(40);
    }

    println!("v: {:?}", v.lock().unwrap());
}
```

Notice how we have a [`impl<T: Send> Sync for Mutex<T>`][2] blanket
implementation.

[1]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
[2]: https://doc.rust-lang.org/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E
[3]: https://doc.rust-lang.org/std/sync/struct.Arc.html

<details>

- `Mutex` in Rust looks like a collection with just one element --- the
  protected data.
  - It is not possible to forget to acquire the mutex before accessing the
    protected data.
- You can get an `&mut T` from an `&Mutex<T>` by taking the lock. The
  `MutexGuard` ensures that the `&mut T` doesn't outlive the lock being held.
- `Mutex<T>` implements both `Send` and `Sync` iff (if and only if) `T`
  implements `Send`.
- A read-write lock counterpart: `RwLock`.
- Why does `lock()` return a `Result`?
  - If the thread that held the `Mutex` panicked, the `Mutex` becomes "poisoned"
    to signal that the data it protected might be in an inconsistent state.
    Calling `lock()` on a poisoned mutex fails with a [`PoisonError`]. You can
    call `into_inner()` on the error to recover the data regardless.

[`PoisonError`]: https://doc.rust-lang.org/std/sync/struct.PoisonError.html

</details>
