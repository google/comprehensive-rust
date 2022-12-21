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
}
```

* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).
