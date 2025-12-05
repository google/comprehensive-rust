---
minutes: 10
---

# forget and drop functions

Below are the signatures for the
[`drop()`](https://doc.rust-lang.org/std/mem/fn.drop.html) and
[`forget()`](https://doc.rust-lang.org/std/mem/fn.forget.html) functions:

```rust
// std::mem::forget
fn forget<T>(t: T) {
    let _ = std::mem::ManuallyDrop::new(t);
}

// std::mem::drop
fn drop<T>(_x: T) {}
```

<details>

- Both `mem::forget()` and `mem::drop()` take ownership of the value `t`.

- Despite having the same function signature, they have opposite effects:

  - `forget()` uses
    [`ManuallyDrop`](https://doc.rust-lang.org/std/mem/struct.ManuallyDrop.html)
    to prevent the destructor `Drop::drop()` from being invoked.

    This is useful for scenarios such as implementing a drop bomb or otherwise
    opting out of destructor behavior.

    Be careful though, since any resources the value exclusively owns such as
    heap allocated memory or file handles will remain in an unreachable state.

  - `drop()` is a convenience function for disposing of a value. Because `t` is
    moved into the function, it is automatically dropped which triggers its
    `Drop::drop()` implementation before the parent function returns.

</details>
