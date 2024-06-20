---
minutes: 5
---

# `Arc`

[`Arc<T>`] is a reference-counted shared pointer to `T`. Use this when you need
to refer to the same data from multiple threads:

```rust,editable
use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(vec![10, 20, 30]);
    let mut handles = Vec::new();
    for _ in 1..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    handles.into_iter().for_each(|h| h.join().unwrap());
    println!("v: {v:?}");
}
```

<details>

- `Arc` is short for "Atomically Reference Counted". It uses atomic
  (thread-safe) CPU instructions to maintain the reference count. `Arc` is a
  thread safe version of [`Rc`].
- `Arc<T>` implements `Clone` whether or not `T` does. It implements `Send` and
  `Sync` if and only if `T` implements them both.
- `Arc::clone()` has the cost of atomic operations that get executed, but after
  that the use of the `T` is free.
- Beware of reference cycles, `Arc` does not use a garbage collector to detect
  them.
  - [`std::sync::Weak`] can help avoid reference cycles.

</details>

[`Arc<T>`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`Rc`]: ../../smart-pointers/rc.md
[`std::sync::Weak`]: https://doc.rust-lang.org/std/sync/struct.Weak.html
