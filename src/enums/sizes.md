# Enum Sizes

Rust enums are packed tightly, taking constraints due to alignment into account:

```rust,editable
use std::mem::{align_of, size_of};

macro_rules! dbg_size {
    ($t:ty) => {
        println!("{}: size {} bytes, align: {} bytes",
                 stringify!($t), size_of::<$t>(), align_of::<$t>());
    };
}

enum Foo {
    A,
    B,
}

fn main() {
    dbg_size!(Foo);
    dbg_size!(bool);
    dbg_size!(Option<bool>);
    dbg_size!(&i32);
    dbg_size!(Option<&i32>);
}
```

* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).

<details>

* `Option<bool>` is another example of tight packing.
* For [some types](https://doc.rust-lang.org/std/option/#representation), Rust guarantees that `size_of::<T>()` equals `size_of::<Option<T>>()`.
* Zero-sized types allow for efficient implementation of `HashSet` using `HashMap` with `()` as the value.

</details>
