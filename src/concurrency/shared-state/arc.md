---
minutes: 5
---

# `Arc`

[`Arc<T>`][1] allows shared, read-only ownership via `Arc::clone`:

```rust,editable
use std::sync::Arc;
use std::thread;

/// A struct that prints which thread drops it.
#[derive(Debug)]
struct WhereDropped(Vec<i32>);

impl Drop for WhereDropped {
    fn drop(&mut self) {
        println!("Dropped by {:?}", thread::current().id())
    }
}

fn main() {
    let v = Arc::new(WhereDropped(vec![10, 20, 30]));
    let mut handles = Vec::new();
    for i in 0..5 {
        let v = Arc::clone(&v);
        handles.push(thread::spawn(move || {
            // Sleep for 0-500ms.
            std::thread::sleep(std::time::Duration::from_millis(500 - i * 100));
            let thread_id = thread::current().id();
            println!("{thread_id:?}: {v:?}");
        }));
    }

    // Now only the spawned threads will hold clones of `v`.
    drop(v);

    // When the last spawned thread finishes, it will drop `v`'s contents.
    handles.into_iter().for_each(|h| h.join().unwrap());
}
```

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html

<details>

- `Arc` stands for "Atomic Reference Counted", a thread safe version of `Rc`
  that uses atomic operations.
- `Arc<T>` implements `Clone` whether or not `T` does. It implements `Send` and
  `Sync` if and only if `T` implements them both.
- `Arc::clone()` has the cost of atomic operations that get executed, but after
  that the use of the `T` is free.
- Beware of reference cycles, `Arc` does not use a garbage collector to detect
  them.
  - `std::sync::Weak` can help.

</details>
