---
minutes: 10
---

# Niche Optimization

For some types, there are in-memory bit patterns that do not represent a valid
value. For example, `bool` can only be 0 or 1, and references are represented as
non-NULL pointers. Rust uses this observation to store enums without a distinct
discriminant field, saving space.

```rust,editable
#![allow(dead_code)]
use std::{mem::size_of_val, slice::from_raw_parts};

fn show<T: Sized>(name: &str, value: T) {
    let bytes = unsafe {
        from_raw_parts(&value as *const T as *const u8, size_of_val(&value))
    };
    println!("{}: {} = {:02x?}", name, std::any::type_name::<T>(), bytes);
}

enum TriState {
    Set(bool),
    Unset,
}

fn main() {
    show("false", TriState::Set(false));
    show("true", TriState::Set(true));
    show("unset", TriState::Unset);
}
```

<details>

The example shows Rust choosing a non-boolean value for the `Unset` variant.

Try showing:

- `&x` for some x
- `Some(&x)`
- `None::<&u32>`
- `Some(Some(&x))`
- `std::num::NonZero::new(10)`

_Null pointer optimization_: For
[some types](https://doc.rust-lang.org/std/option/#representation), Rust
guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()` and that the
all-zeroes pattern transmutes to `None`. This is a special-case of the niche
optimization for `Option`.

</details>
