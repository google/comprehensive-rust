# MaybeUninit.write() vs assignment

```rust
use std::mem::MaybeUninit;

fn main() {
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

    let external_data = b"Hello, Rust!";

    for (dest, src) in buf.iter_mut().zip(external_data) {
        // *dest = MaybeUninit::new(*src)

        // W
        dest.write(*src);
    }

    todo!()
}
```

<details>

“When writing data with `MaybeUninit.write()`, the old value is not dropped.

“`MaybeUninit` does not call the destructor on its value, because the compiler
cannot guarantee that the value has been properly initialized.

“This is different than what occurs on assignment. Assignment triggers a move,
which results in a bitwise copy. This can trigger memory leaks.

</details>
