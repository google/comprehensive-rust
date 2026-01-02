# Arrays of uninit

```rust
use std::mem::{MaybeUninit, size_of};

fn main() {
    let stack = [const { MaybeUninit::<u8>::uninit() }; 2048];
    assert!(std::size_of)
    todo!("add size of assertion [u8; 2048]")

    let heap = Box::new_uninit::<[u8; 2048]>();

}
```

<details>

Rust supports some shorthand forms to create arrays of uninit.

</details>
