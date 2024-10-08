---
minutes: 3
---

# Empty Containers

Idiomatically, Rust containers do not allocate any memory when empty.

```rust,editable
#![allow(dead_code)]
use std::{mem::size_of_val, slice::from_raw_parts};

fn show<T: Sized>(name: &str, value: T) {
    let bytes = unsafe {
        from_raw_parts(&value as *const T as *const u8, size_of_val(&value))
    }
    .iter()
    .map(|b| format!("{:02x}", b))
    .collect::<Vec<_>>()
    .join("");
    println!("{}: {} = {}", name, std::any::type_name::<T>(), bytes);
}

fn main() {
    show("empty", String::new());
    show("nonempty", String::from("nonempty"));
}
```

<details>

The `show` function is the same as the previous slide. Try also showing an empty
`Vec`. What about an allocated ZST like `Box<()>`?

Note too, for people thinking about C++ interoperability, that these empty
containers are not represented by an all-zeros pattern. These types use
[`std::ptr::NonNull::dangling()`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.dangling)
to represent an invalid pointer, reserving NULL for the niche optimization.

</details>
