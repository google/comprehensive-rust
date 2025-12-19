# Partial Initialization

```rust
use std::mem::MaybeUninit;

fn partial_init(buf: &mut [MaybeUninit::<u8>]) -> &str {
    let data = b"Hello, Rust!";

    for (dest, src) in buf.iter_mut().zip(external_data) {
        dest.write(*src);
    }

    // Create a slice of initialized bytes, rather than 
    // assume_init on the whole array

    // SAFETY: 0..data.len() bytes are initialized and contain valid UTF-8 data
    unsafe {
        let start = buf.as_mut_ptr();
        let slice = std::slice::from_raw_parts_mut(start, data.len());
        str::from_utf8(slice)
    };
}

fn main() {
    let mut buf = [const { MaybeUninit::<u8>::uninit() }; 2048];

    let init = partial_init(buf)

    println!("{}", String::from_utf8_lossy(init));
}
```

<details>

This example shows the use of

</details>
