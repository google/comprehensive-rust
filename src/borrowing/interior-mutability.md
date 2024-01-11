---
minutes: 10
---

# Interior Mutability

In some situations, exclusive (`mut`) access cannot be represented at compile
time. For example, data might be accessed from multiple threads. The "interior
mutability" pattern allows exclusive access behind a shared reference, using
a runtime check to enforce the borrowing rules.

## `Mutex`

One standard type that supports interior mutability is `Mutex` (known as a lock
in some other languages). Many threads can have a shared reference to a
`Mutex`, and the lock operation performs a _runtime_ check that no other thread
holds the lock before granting access to the value inside.

```rust,editable
use std::sync::Mutex;

fn handle_hit(hit_counter: &Mutex<u32>) {
    let mut num_hits = hit_counter.lock().unwrap();
    *num_hits += 1;
}

fn current_hit_count(hit_counter: &Mutex<u32>) -> u32 {
    *hit_counter.lock().unwrap()
}

fn main() {
    let hit_counter = Mutex::new(0);
    handle_hit(&hit_counter);
    println!("{} hits", current_hit_count(&hit_counter));
}
```

## `Cell` and `RefCell`

`Cell` wraps a value and allows getting or setting the value, even with a
shared reference to the `Cell`. Howerver, it does not allow any references to
the value. If there are no references, then borrowing rules cannot be broken.

The `RefCell` type also provides interior mutability. Its `borrow` method
allows multiple shared references to the data it contains, while its
`borrow_mut` allows a single exclusive reference. It checks the borrowing rules
at runtime and panics if they are violated.

Neither `Cell` nor `RefCell` can be shared between threads.

<details>

- While the fundamentals course doesn't cover concurrency, most students will
  have seen a mutex or lock before, and understand that it checks for an existing
  lock at runtime, which is the key to interior mutability.

- The cell types are more unusual, if simpler.  `RefCell` is, in effect, a
  `RWMutex` that panics instead of blocking.

</details>
