# MaybeUninit and arrays

```rust
use std::mem::MaybeUninit;

fn main() {
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];
}
```

<details>

Uninitialized memory often arrives.

Use `ptr::write` to initialize.

</details>
