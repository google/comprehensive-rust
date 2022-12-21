# Shared State

Rust uses the type system to enforce synchronization of shared data. This is
primarily done via two types:

* [`Arc<T>`][1], atomic reference counted `T`: handled sharing between threads and
  takes care to deallocate `T` when the last thread exists,
* [`Mutex<T>`][2]: ensures mutual exclusion for to the `T` value.

[1]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[2]: https://doc.rust-lang.org/std/sync/struct.Mutex.html
