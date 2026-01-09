# MaybeUninit.write() vs assignment

```rust
use std::mem::MaybeUninit;

fn main() {
    let mut buf = MaybeUninit::<String>::uninit();

    // Initialize
    buf.write(String::from("Hello, Rust!"));

    // Overwrite
    buf.write(String::from("Hi again"));

    // Assignment replaces the whole MaybeUninit value.
    buf = MaybeUninit::new(String::from("Goodbye"));

    // Ensure inner value is dropped
    let _ = unsafe { buf.assume_init() };
}
```

<details>

Replacing inner values can cause memory leaks because the drop semantics differ
from most types. `MaybeUninit<T>` does not call the destructor on its `T`.

`MaybeUninit::write()` uses `ptr::write`: it initializes the memory in place
without reading or dropping the old contents. That is exactly what you want when
the memory might be uninitialized, but it also means you will leak if there was
already a live value there.

Assignment, e.g. `buf = MaybeUninit::new(value)`, replaces the whole
`MaybeUninit`. The old `MaybeUninit` is moved and then dropped, but
`MaybeUninit` has no destructor for `T`, so the inner value is not dropped. If
the old slot held an initialized value, it is leaked just like with `write()`.

If you need normal drop behavior, you need to tell Rust that the value is
initialized with `assume_init` or one of the related methods.

</details>
