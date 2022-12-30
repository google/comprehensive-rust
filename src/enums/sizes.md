# Enum Sizes

러스트의 열거형은 정렬로 인한 제약을 고려하여 크기를 빽빽하게 잡습니다
> Rust enums are packed tightly, taking constraints due to alignment into account:

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

- 자세한 사항은 [공식문서](https://doc.rust-lang.org/reference/type-layout.htm)를 확인하세요.
* See the [Rust Reference](https://doc.rust-lang.org/reference/type-layout.html).
