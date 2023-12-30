# `spin`

`std::sync::Mutex` and the other synchronisation primitives from `std::sync` are
not available in `core` or `alloc`. How can we manage synchronisation or
interior mutability, such as for sharing state between different CPUs?

The [`spin`][1] crate provides spinlock-based equivalents of many of these
primitives.

<!-- mdbook-xgettext: skip -->

```rust,editable,compile_fail
use spin::mutex::SpinMutex;

static counter: SpinMutex<u32> = SpinMutex::new(0);

fn main() {
    println!("count: {}", counter.lock());
    *counter.lock() += 2;
    println!("count: {}", counter.lock());
}
```

<details>

- Be careful to avoid deadlock if you take locks in interrupt handlers.
- `spin` also has a ticket lock mutex implementation; equivalents of `RwLock`,
  `Barrier` and `Once` from `std::sync`; and `Lazy` for lazy initialisation.
- The [`once_cell`][2] crate also has some useful types for late initialisation
  with a slightly different approach to `spin::once::Once`.
- The Rust Playground includes `spin`, so this example will run fine inline.

</details>

[1]: https://crates.io/crates/spin
[2]: https://crates.io/crates/once_cell
