---
minutes: 6
---

# Examples

## `Send + Sync`

Most types you come across are `Send + Sync`:

- `i8`, `f32`, `bool`, `char`, `&str`, ...
- `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, ...
- `String`, `Option<T>`, `Vec<T>`, `Box<T>`, ...
- `Arc<T>`: Explicitly thread-safe via atomic reference count.
- `Mutex<T>`: Explicitly thread-safe via internal locking.
- `mpsc::Sender<T>`: As of 1.72.0.
- `AtomicBool`, `AtomicU8`, ...: Uses special atomic instructions.

The generic types are typically `Send + Sync` when the type parameters are
`Send + Sync`.

## `Send + !Sync`

These types can be moved to other threads, but they're not thread-safe.
Typically because of interior mutability:

- `mpsc::Receiver<T>`
- `Cell<T>`
- `RefCell<T>`

## `!Send + Sync`

These types are safe to access (via shared references) from multiple threads,
but they cannot be moved to another thread:

- `MutexGuard<T: Sync>`: Uses OS level primitives which must be deallocated on
  the thread which created them. However, an already-locked mutex can have its
  guarded variable read by any thread with which the guard is shared.

## `!Send + !Sync`

These types are not thread-safe and cannot be moved to other threads:

- `Rc<T>`: each `Rc<T>` has a reference to an `RcBox<T>`, which contains a
  non-atomic reference count.
- `*const T`, `*mut T`: Rust assumes raw pointers may have special concurrency
  considerations.
