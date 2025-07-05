# Less powerful than it seems

The `unsafe` keyword

```rust
use std::mem::transmute;

let orig = b"RUST";
let n: i32 = unsafe { transmute(orig) };

println!("{}")
```

<details>

Try to c different

</details>
