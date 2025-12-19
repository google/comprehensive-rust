---
minutes: 8
---

# MaybeUninit<T>

`MaybeUninit<T>` allows Rust to refer to uninitialized memory.

```rust,editable
use std::mem::MaybeUninit;

fn main() {
    let uninit = MaybeUninit::<&i32>::uninit();
    println!("{uninit:?}");
}
```

<details>

“Safe Rust is unable to refer to data that’s potentially uninitialized”

“Yet, all data arrives at the program as uninitialized.”

“Therefore, we need some bridge in the type system to allow memory to
transition. `MaybeUninit<T>` is that type.”

It is very similar to the `Option<T>` type, although its semantics are very
different. Its equivalent of None,uninitialized.

Reading from memory that may be uninitialized is extremely dangerous.

</details>
