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
    };
    println!("{}: {} = {:02x?}", name, std::any::type_name::<T>(), bytes);
}

fn main() {
    show("empty", String::new());
    show("nonempty", String::from("nonempty"));
}
```

<details>

The `show` function is the same as the previous slide. We can see that the
nonempty value has some nonzero bytes that look like a pointer.

Try also showing an empty `Vec` (`Vec::<u32>::new()`). What about an allocated
ZST like `Box<()>`?

Note that the empty containers are not represented by an all-zeros pattern.
These types use
[`std::ptr::NonNull::dangling()`](https://doc.rust-lang.org/beta/std/ptr/struct.NonNull.html#method.dangling)
to represent an invalid pointer, reserving NULL for the niche optimization in
e.g., `Option<Vec<T>>`. Internally, implementations are careful to check that
`capacity` is nonzero before assuming that the pointer is valid.

</details>
